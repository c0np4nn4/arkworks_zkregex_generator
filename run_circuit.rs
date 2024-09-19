use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::{
    alloc::AllocVar,
    boolean::Boolean,
    eq::EqGadget,
    fields::{fp::FpVar, FieldVar}, //FpVar::constant을 이용하기 위해 추가
};

/// Regex: a[bc]+d
#[derive(Clone)]
struct RegexCircuit<F: PrimeField> {
    input: Vec<F>,
    max_len: usize,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for RegexCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // 입력 변수 초기화
        let input_vars = self
            .input
            .into_iter()
            .map(|v| FpVar::new_input(cs.clone(), || Ok(v)))
            .collect::<Result<Vec<_>, _>>()?;

        let mut valid = Boolean::constant(true);

        // 상태 변수 초기화 (4개의 상태)
        if input_vars.len() < 3 || input_vars.len() > 4 {
            valid = Boolean::constant(false);
        } else {
            let mut current_state = FpVar::constant(F::from(0u64));

            // 각 입력 인덱스에 대한 전이 로직
            for current_input in input_vars.iter() { //for문 사용으로 코드 중복 최소화
                let mut next_state = current_state.clone(); // let mut next_sate = None은 Option<FpVar<F>>로 설정 // 그래서 타입 불일치 발생 // Option이 아닌 FpVar<F> 타입으로 선언하고, current_state를 복제하여 초기화

                // 상태에 따른 전이 정의
                // 상태 0
                let is_state_0 = current_state.is_eq(&FpVar::constant(F::from(0u64)))?; //회로 내에서는 if 문을 사용할 수 없기 때문에, Boolean 타입의 메서드(and, or, not, select)를 사용
                
                // 상태 1
                let is_state_1 = current_state.is_eq(&FpVar::constant(F::from(1u64)))?;
                // 상태 2
                let is_state_2 = current_state.is_eq(&FpVar::constant(F::from(2u64)))?;

                // 입력 값과 비교
                let is_input_97 = current_input.is_eq(&FpVar::constant(F::from(97u64)))?;
                let is_input_98 = current_input.is_eq(&FpVar::constant(F::from(98u64)))?;
                let is_input_99 = current_input.is_eq(&FpVar::constant(F::from(99u64)))?;
                let is_input_100 = current_input.is_eq(&FpVar::constant(F::from(100u64)))?;

                // 상태 전이 로직
                // 상태 0에서
                let cond0 = is_state_0.and(&is_input_97)?;
                next_state = cond0.select(&FpVar::constant(F::from(1u64)), &next_state)?;

                // 상태 1에서
                let cond1a = is_state_1.and(&is_input_98)?;
                let cond1b = is_state_1.and(&is_input_99)?;
                let cond1 = cond1a.or(&cond1b)?;
                next_state = cond1.select(&FpVar::constant(F::from(2u64)), &next_state)?;

                // 상태 2에서 (반복되는 상태)
                let cond2a = is_state_2.and(&is_input_98)?;
                let cond2b = is_state_2.and(&is_input_99)?;
                let cond2 = cond2a.or(&cond2b)?;
                next_state = cond2.select(&FpVar::constant(F::from(2u64)), &next_state)?;

                // 상태 2에서 종료 상태로
                let cond3 = is_state_2.and(&is_input_100)?;
                next_state = cond3.select(&FpVar::constant(F::from(3u64)), &next_state)?;

                // 전이가 발생하지 않은 경우 유효하지 않음
                let state_changed = current_state.is_eq(&next_state)?.not();
                valid = valid.and(&state_changed)?;

                // 현재 상태 업데이트
                current_state = next_state;
            }

            // 수락 상태 확인
            let is_accepting = current_state.is_eq(&FpVar::constant(F::from(3u64)))?;
            valid = valid.and(&is_accepting)?;
        }

        // 최종적으로 valid가 true인지 확인
        valid.enforce_equal(&Boolean::constant(true))?;
        Ok(())
    }
}
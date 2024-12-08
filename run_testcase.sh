#!/bin/bash

# jq 설치 확인 및 설치 (만약 설치되어 있지 않은 경우)
if ! command -v jq &> /dev/null; then
    echo "jq not found. Installing jq..."
    sudo apt-get install -y jq
fi

# gtime 설치 확인 및 설치 (만약 설치되어 있지 않은 경우)
if ! command -v gtime &> /dev/null; then
    echo "gtime not found. Installing gtime..."
    sudo apt-get install -y gnu-time
fi

# 입력 파일과 출력 파일 정의
# INPUT_FILE="dfa_test.json"
INPUT_FILE="ttt.json"
OUTPUT_FILE="test_result.csv"

# CSV 파일 헤더 작성
echo "Regex,Test String,Expected Result,Actual Result,Time Taken (seconds)" > $OUTPUT_FILE

# JSON 파일을 읽어와서 반복 처리
for i in $(jq -c '.[]' $INPUT_FILE); do
    # JSON에서 regex, pass, fail 추출
    regex=$(echo $i | jq -r '.regex')
    pass_cases=$(echo $i | jq -r '.pass[]')
    fail_cases=$(echo $i | jq -r '.fail[]')

    # Pass 케이스 실행
    for test_case in $pass_cases; do
        echo "Testing (pass) case: $test_case"

        # regex와 test string을 파일로 저장
        echo "$regex" > regex.txt
        echo "$test_case" > test_string.txt
        
        # 시간 측정 시작 (gtime으로 시간만 추출)
        gtime -f "%e" -o temp_time.txt bash -c "
            cargo run < regex.txt > /dev/null 2>&1;  # 첫 번째 cargo run - regex로 circuit 생성
            cargo run --manifest-path=example/Cargo.toml < test_string.txt > /dev/null 2>&1;  # 두 번째 cargo run - test_string 검증
        "
        time_taken=$(cat temp_time.txt)

        # 성공 여부를 종료 상태로 확인
        if [ $? -eq 0 ]; then
            actual_result="PASS"
        else
            actual_result="FAIL"
        fi

        # 결과 저장
        echo "$regex,\"$test_case\",true,$actual_result,$time_taken" >> $OUTPUT_FILE
    done

    # Fail 케이스 실행
    for test_case in $fail_cases; do
        echo "Testing (fail) case: $test_case"

        # regex와 test string을 파일로 저장
        echo "$regex" > regex.txt
        echo "$test_case" > test_string.txt
        
        # 시간 측정 시작 (gtime으로 시간만 추출)
        gtime -f "%e" -o temp_time.txt bash -c "
            cargo run < regex.txt > /dev/null 2>&1;  # 첫 번째 cargo run - regex로 circuit 생성
            cargo run --manifest-path=example/Cargo.toml < test_string.txt > /dev/null 2>&1;  # 두 번째 cargo run - test_string 검증
        "
        time_taken=$(cat temp_time.txt)

        # 성공 여부를 종료 상태로 확인
        if [ $? -ne 0 ]; then
            actual_result="PASS"
        else
            actual_result="FAIL"
        fi

        # 결과 저장
        echo "$regex,\"$test_case\",false,$actual_result,$time_taken" >> $OUTPUT_FILE
    done
done

# 임시 파일 삭제
rm temp_time.txt
rm regex.txt test_string.txt

echo "Test completed. Results saved in $OUTPUT_FILE"


#!/bin/bash

# jq 설치 확인 및 설치 (만약 설치되어 있지 않은 경우)
if ! command -v jq &> /dev/null; then
    echo "jq not found. Installing jq..."
    sudo apt-get install -y jq
fi

# gtime 설치 확인 및 설치 (만약 설치되어 있지 않은 경우)
if ! command -v gtime &> /dev/null; then
    echo "gtime not found. Installing gtime..."
    sudo apt-get install -y gnu-time
fi

# 입력 파일과 출력 파일 정의
INPUT_FILE="dfa_test.json"
OUTPUT_FILE="test_result.csv"

# CSV 파일 헤더 작성
echo "Regex,Test String,Expected Result,Actual Result,Time Taken (seconds)" > $OUTPUT_FILE

# JSON 파일을 읽어와서 반복 처리
for i in $(jq -c '.[]' $INPUT_FILE); do
    # JSON에서 regex, pass, fail 추출
    regex=$(echo $i | jq -r '.regex')
    pass_cases=$(echo $i | jq -r '.pass[]')
    fail_cases=$(echo $i | jq -r '.fail[]')

    # Pass 케이스 실행
    for test_case in $pass_cases; do
        echo "Testing (pass) case: $test_case"

        # regex와 test string을 파일로 저장
        echo "$regex" > regex.txt
        echo "$test_case" > test_string.txt
        
        # 시간 측정 시작 (gtime으로 시간만 추출)
        gtime -f "%e" -o temp_time.txt bash -c "
            cargo run < regex.txt > /dev/null 2>&1;  # 첫 번째 cargo run - regex로 circuit 생성
            cargo run --manifest-path=example/Cargo.toml < test_string.txt > /dev/null 2>&1;  # 두 번째 cargo run - test_string 검증
        "
        
        time_taken=$(cat temp_time.txt)

        # 명령어 성공/실패 여부 확인
        if [ $? -eq 0 ]; then
            actual_result="PASS"
        else
            actual_result="FAIL"
        fi

        # 결과 저장 (실행 시간만 기록)
        echo "$regex,\"$test_case\",true,$actual_result,$time_taken" >> $OUTPUT_FILE
    done

    # Fail 케이스 실행
    for test_case in $fail_cases; do
        echo "Testing (fail) case: $test_case"

        # regex와 test string을 파일로 저장
        echo "$regex" > regex.txt
        echo "$test_case" > test_string.txt
        
        # 시간 측정 시작 (gtime으로 시간만 추출)
        gtime -f "%e" -o temp_time.txt bash -c "
            cargo run < regex.txt > /dev/null 2>&1;  # 첫 번째 cargo run - regex로 circuit 생성
            cargo run --manifest-path=example/Cargo.toml < test_string.txt > /dev/null 2>&1;  # 두 번째 cargo run - test_string 검증
        "

        time_taken=$(cat temp_time.txt)

        # 명령어 성공/실패 여부 확인
        if [ $? -ne 0 ]; then
            actual_result="PASS"
        else
            actual_result="FAIL"
        fi

        # 결과 저장 (실행 시간만 기록)
        echo "$regex,\"$test_case\",false,$actual_result,$time_taken" >> $OUTPUT_FILE
    done
done

# 임시 파일 삭제
rm temp_time.txt
rm regex.txt test_string.txt

echo "Test completed. Results saved in $OUTPUT_FILE"


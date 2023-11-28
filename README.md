# Projump

![rust logo](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=Rust&logoColor=white)

### Projump
> project + jump 의 합성어로, 자주 접근하는 개발 프로젝트의 working directory 에 alias 로 빠르게 접근할 수 있도록 도와주는 CLI 프로그램

## Usage

### 1. 프로젝트 Alias 설정하기
```bash
projump . -a myproject1

# or

projump -a myproject1
```

* 현재 경로와 `myproject1` 이라는 alias 를 연결함
* 다른 경로에서 동일한 이름으로 등록을 시도하면 에러 반환
* `--force` 옵션으로 덮어쓰기 가능
* 절대 경로 설정도 가능
    ```bash
    projump /abolute/path/to/myproject1 -a myproject1
    ```


### 2. 프로젝트 위치로 jump 하기
```bash
projump myproject1
```


### 3. 저장한 alias 의 목록 보기
```bash
projump ls
```

### 4. 저장되어있는 alias 삭제하기
```bash
projump -d myproject1
```

### 5. 저장되어있는 alias 이름 변경하기
```bash
projump -m myproject1 myproject2
```

## Notes
Rust 프로그램에서 현재 경로를 바꾸어도 해당 프로세스는 별도의 shell 에서 이루어지기 때문에 현재 사용자가 보고 있는 shell 의 현재 디렉토리가 변경되지 않는 문제가 있음.

그렇기 때문에 Rust 프로그램을 호출하는 shell function 을 만들어서 이 함수를 호출하는 방식으로 구현함. 아래는 `.zshrc` 파일에 projump shell function 을 구현한 코드임.

```bash
function projump {
    rust_executable="/Absolute/path/to/projump/exe/file"
    if [ "$#" -eq 1 ]; then
        data_file="/Absolute/path/to/projump/alias/file"
        alias_name="$1"
        directory=$(grep "^$alias_name " "$data_file" | awk '{print $2}')
        if [ -n "$directory" ]; then
            echo "$directory"
            cd "$directory" || echo "No such directory."
        else
            echo "No such alias found."
        fi
    else 
        "$rust_executable" "$@"
fi
}
```
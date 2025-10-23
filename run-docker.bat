@echo off
REM 기존 컨테이너가 실행 중이면 강제 삭제
docker rm -f parking_server_rust

REM ocr data copy
REM copy ocr.proto parking_server\ocr.proto

REM 긴 도커 명령어 입력. Windows 환경변수 %cd%를 사용하여 현재 경로를 마운트합니다.
docker run ^
  --rm ^
  --name parking_server_rust ^
  --network my_mqtt_net ^
  --network mssql_network ^
  --network ocr_server ^
  -v "%cd%:/app" ^
  --workdir "/app" ^
  parking_server ^
  cargo run
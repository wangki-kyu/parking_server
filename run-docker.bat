@echo off
REM 기존 컨테이너가 실행 중이면 강제 삭제
docker rm -f parking_server

REM 긴 도커 명령어 입력. Windows 환경변수 %cd%를 사용하여 현재 경로를 마운트합니다.
docker run ^
  --rm ^
  --name parking_server ^
  --network my_mqtt_net ^
  -v "%cd%:/app" ^
  --workdir "/app" ^
  parking_server ^
  cargo run
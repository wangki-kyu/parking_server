# ocr_script.py
import easyocr
import sys
import os

# 명령줄 인수로 이미지 파일 경로를 받습니다.
if len(sys.argv) < 2:
    print("사용법: python ocr_script.py <이미지_파일_경로>")
    sys.exit(1)

image_path = sys.argv[1]
# 인식할 언어를 설정합니다 (예: 한국어, 영어).
# Dockerfile에서 EasyOCR을 설치했으므로 별도의 설치는 필요 없습니다.
reader = easyocr.Reader(['ko', 'en'])

print(f"이미지 경로: {image_path}")
print("--- OCR 결과 ---")

try:
    # OCR 실행
    # detail=0으로 설정하면 (bbox, text, confidence) 대신 text만 반환됩니다.
    result = reader.readtext(image_path, detail=0)

    for text in result:
        print(text)

except Exception as e:
    print(f"OCR 실행 중 오류가 발생했습니다: {e}")

# (선택 사항) 결과를 파일로 저장하고 싶다면 여기에 추가
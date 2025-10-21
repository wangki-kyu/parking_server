# import warnings
import sys
import os
from paddleocr import PaddleOCR
import logging

# 시스템 경고 메시지 (pkg_resources 관련)를 숨깁니다.
# logger = logging.getLogger('ppocr')
# logger.setLevel(logging.CRITICAL)
# warnings.filterwarnings('ignore', category=DeprecationWarning)
logger = logging.getLogger('ppocr')
logger.setLevel(logging.CRITICAL) 

# 명령줄 인수로 이미지 파일 경로를 받습니다.
if len(sys.argv) < 2:
    print("사용법: python ocr_script.py <이미지_파일_경로>")
    sys.exit(1)

image_path = sys.argv[1]

# OCR 객체를 초기화합니다. (lang="korean" 설정 유지)
# 모델은 이미 Docker 빌드 시 캐싱되었으므로 빠르게 로드됩니다.
ocr = PaddleOCR(use_angle_cls=True, lang="korean", use_gpu=False)

# 입력 이미지 파일 경로 설정
# IMG_DIR = "input_images"
# # 사용자님의 이미지 파일명에 맞게 이 부분을 수정하세요!
# INPUT_IMAGE_NAME = "sample_korean_text.jpg" 
# IMAGE_PATH = os.path.join(IMG_DIR, INPUT_IMAGE_NAME)

def run_ocr_and_combine(image_path, ocr_engine):
    combined_text = ""
    try:
        # OCR 실행: 텍스트 감지 및 인식
        # result 구조: [ [좌표, (텍스트, 신뢰도)], ... ]
        result = ocr_engine.ocr(image_path, det=True, rec=True)

        if result and result[0] is not None:
            for line in result[0]:
                # line[1][0]은 인식된 텍스트입니다.
                text = line[1][0]
                
                # 공백을 제거합니다. (EasyOCR의 readtext(detail=0) 후처리 방식 모방)
                cleaned_text = text.replace(" ", "")
                combined_text += cleaned_text
        
        # 최종 결과를 표준 출력으로 내보냅니다.
        print(combined_text)

    except Exception as e:
        # 오류 발생 시 출력
        print(f"OCR 실행 중 오류가 발생했습니다: {e}")
        # 오류가 발생해도 시스템 경고와 섞이지 않도록 에러 메시지만 출력합니다.

# --- 메인 실행 ---
if not os.path.exists(image_path):
    print(f"오류: '{image_path}' 파일을 찾을 수 없습니다. 경로를 확인해 주세요.")
else:
    run_ocr_and_combine(image_path, ocr)

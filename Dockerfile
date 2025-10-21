FROM paddlecloud/paddleocr:2.6-cpu-latest

# 작업 디렉토리
WORKDIR /app

# rust 의존성 설치
RUN apt-get update && \
    apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 2. Rust 설치 명령어 추가 (사용자 제공)
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.86

# 3. 환경 변수 설정 (사용자 제공)
ENV PATH="/root/.cargo/bin:${PATH}"

RUN python -m pip install paddlepaddle -i https://pypi.tuna.tsinghua.edu.cn/simple
RUN python -m pip install paddleocr
RUN python -c "from paddleocr import PaddleOCR; PaddleOCR(use_angle_cls=True, lang='korean').ocr('dummy.jpg', det=False, rec=False, cls=False)" || echo 'Dummy run for model cache complete'

# COPY my_ocr_script.py .
# COPY input_images/ ./input_images/

CMD ["python", "my_ocr_script.py"]


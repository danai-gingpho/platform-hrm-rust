# API Gateway (Rust)

ระบบ API Gateway ประสิทธิภาพสูงที่พัฒนาด้วยภาษา Rust โดยใช้เฟรมเวิร์ก Axum ออกแบบมาเพื่อทำหน้าที่เป็นด่านหน้า (Entry Point) สำหรับระบบ Microservices โดยเน้นความปลอดภัย ความทนทาน และประสิทธิภาพ

## 🚀 ฟีเจอร์หลัก (Key Features)

-   **Authentication & Authorization**: เชื่อมต่อกับ Keycloak (OIDC) เพื่อตรวจสอบ JWT Token
-   **Fine-grained Access Control**: กำหนดสิทธิ์การเข้าถึงแต่ละ Route ได้ตาม Roles และ Scopes
-   **Body Streaming**: รองรับการส่งต่อข้อมูล (Request/Response) แบบ Streaming ประหยัดหน่วยความจำแม้จัดการไฟล์ขนาดใหญ่
-   **Rate Limiting**: ระบบจำกัดความเร็วราย IP (Per-IP Rate Limit) ป้องกันการโจมตีแบบ DoS และการใช้งานเกินขีดจำกัด
-   **Resilience (Retry Logic)**: ระบบส่งคำขอใหม่ivอัตโนมัติเมื่อเกิดความผิดพลาดชั่วคราว (502, 503, 504) สำหรับคำขอที่ปลอดภัย (GET, HEAD, OPTIONS)
-   **Identity Propagation**: แทรกข้อมูลผู้ใช้ (Subject, Username, Email, Roles) ลงใน HTTP Headers ก่อนส่งต่อให้ Service หลังบ้าน
-   **Observability**: 
    -   มีระบบ **Distributed Tracing** และ **Request ID**
    -   มี **Prometheus Metrics** พร้อมใช้งานที่หน้า `/metrics`
-   **Reverse Proxy**: ส่งต่อคำขอไปยัง Upstream Services ตามการตั้งค่าในไฟล์คอนฟิก

## 🏗️ สถาปัตยกรรม (Architecture)

โปรเจกต์นี้ใช้โครงสร้างแบบ **Clean Architecture**:
-   `domain/`: กฎทางธุรกิจหลักและโมเดลข้อมูล
-   `application/`: ตรรกะการตรวจสอบสิทธิ์และการจัดการนโยบาย
-   `infrastructure/`: การเชื่อมต่อภายนอก (HTTP Client, Keycloak, JWKS)
-   `interface/`: ส่วนติดต่อผู้ใช้ (HTTP Server, Middleware, Proxy Handlers)

## 🛠️ เริ่มต้นใช้งาน (Getting Started)

### ความต้องการของระบบ (Prerequisites)
-   Rust (ล่าสุด)
-   Docker และ Docker Compose (สำหรับการรัน Keycloak และทดสอบ)

### การติดตั้ง (Installation)
1. เตรียมไฟล์คอนฟิกและ Environment:
   ```bash
   make env
   ```
2. แก้ไขไฟล์ `config.toml` และ `.env` ตามสภาพแวดล้อมของคุณ

### การรันระบบ (Running)
- **รันด้วย Docker (ครบชุดรวม Keycloak):**
  ```bash
  make up
  ```
- **รัน Gateway ในเครื่อง (Development):**
  ```bash
  make run
  ```

## 📊 การตรวจสอบและติดตาม (Monitoring)

-   **Health Check**: `curl http://localhost:8000/health`
-   **Metrics (Prometheus)**: `curl http://localhost:8000/metrics` หรือใช้คำสั่ง `make metrics`

## 🛠️ คำสั่งที่ใช้บ่อย (Useful Makefile Commands)

-   `make build`: คอมไพล์โปรเจกต์
-   `make test`: รัน Automated Tests
-   `make lint`: ตรวจสอบคุณภาพโค้ด (Clippy)
-   `make tidy`: จัดฟอร์แมตและตรวจสอบโค้ด
-   `make logs`: ดู Log ของ Gateway
-   `make down`: หยุดการทำงานของ Docker containers

## 📝 License
[กำหนด License ของคุณที่นี่]

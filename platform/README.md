# HRM Platform Microservice

โปรเจกต์นี้เป็นโครงสร้างต้นแบบสำหรับระบบ HRM ที่รองรับ Multi-tenancy แบบ 1 Company = 1 Schema โดยใช้ Rust

## คุณสมบัติหลัก
- **Multiplexing:** รัน gRPC และ REST API บนพอร์ตเดียวกัน (8080)
- **Multi-tenancy:** รองรับการสลับ Schema ของ PostgreSQL โดยอัตโนมัติผ่าน Middleware (`search_path`)
- **Keycloak Ready:** มี Middleware โครงสร้างสำหรับตรวจสอบ JWT และดึง Company Context
- **Sea-ORM:** ใช้ Sea-ORM สำหรับจัดการฐานข้อมูล

## โครงสร้างโปรเจกต์
- `proto/`: ไฟล์ definition สำหรับ gRPC
- `src/domain/`: เก็บ Entity และความสัมพันธ์ (Central Schema)
- `src/middleware/`: จัดการเรื่อง Auth และ Company switching
- `src/application/`: Business Logic (เช่น การสร้าง Schema ใหม่เมื่อมี Company ใหม่)
- `src/interface/`: ส่วนเชื่อมต่อภายนอก (gRPC/HTTP Handlers)

## วิธีการใช้งาน
1. ตั้งค่า Database URL ใน Environment Variable หรือใช้ค่าเริ่มต้น
2. รันโปรเจกต์ด้วย `cargo run`

```bash
cd platform
cargo run
```

## สถาปัตยกรรม Multi-tenancy
เมื่อมีการส่ง Request พร้อม Header `x-company-id`:
1. `auth_middleware` จะตรวจสอบสิทธิ์
2. `company_middleware` จะสั่ง `SET search_path TO company_xxxx, public`
3. Database Query หลังจากนั้นจะวิ่งเข้าหา Schema ของบริษัทนั้นๆ โดยอัตโนมัติ

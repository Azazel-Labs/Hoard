## ข้อผิดพลาดของเซิร์ฟเวอร์

error-account-not-found = ไม่พบบัญชี
error-asset-gone = ไม่พบแอสเซ็ต
error-asset-not-found = ไม่พบแอสเซ็ต { $id }
error-blob-not-found = ไม่พบบล็อบ { $hash }
error-collection-not-found = ไม่พบคอลเลกชัน { $id }
error-connection-not-found = ไม่พบการเชื่อมต่อ
error-derivative-not-found = ไม่พบไฟล์ { $kind } ที่สร้างจาก { $id }
error-file-in-pack-not-found = ไม่พบ { $path } ในแพ็ก { $id }
error-site-not-a-page = เปิดเป็นหน้าได้เฉพาะหน้าเว็บเท่านั้น
error-site-link-expired = ที่อยู่ของหน้านี้หมดอายุแล้ว เปิดหน้านี้อีกครั้งจาก Hoard
error-format-not-found = ไม่พบรูปแบบ { $file }
error-frame-sheet-aseprite-only = มีเพียงไฟล์ Aseprite ที่มีแผ่นเฟรม
error-frame-sheet-frames = ไฟล์นี้มีเฟรมเดียว หรือมีเฟรมมากเกินกว่าจะจัดวางได้
error-location-not-found = ไม่พบตำแหน่งจัดเก็บ { $id }
error-maintenance-task-not-found = ไม่พบงานบำรุงรักษา { $id }
error-open-upload-not-found = ไม่พบการอัปโหลดที่เปิดอยู่ { $id }
error-original-not-found = ไม่พบไฟล์ต้นฉบับของ { $id }
error-pack-not-found = ไม่พบแพ็ก { $id }
error-preview-too-big = ภาพนี้ใหญ่เกินกว่าจะสร้างตัวอย่างแบบเบาได้
error-project-not-found = ไม่พบโปรเจกต์ { $name }
error-saved-search-not-found = ไม่พบการค้นหาที่บันทึกไว้ { $id }
error-sign-in-first = ลงชื่อเข้าใช้ก่อน (หรือส่งโทเค็นการเข้าถึง)
error-those-assets-not-found = ไม่พบแอสเซ็ตเหล่านั้น
error-thumbnail-not-found = ไม่พบภาพย่อของ { $id }
error-token-not-found = ไม่พบโทเค็น
error-upload-not-found = ไม่พบการอัปโหลด { $id }
error-pack-needs-name = แพ็กต้องมีชื่อ
error-no-library = ยังไม่มีตำแหน่งคลังสำหรับจัดเก็บไฟล์: เพิ่มตำแหน่งที่ Hoard จัดการได้ที่ การตั้งค่า → ตำแหน่งคลัง
error-upload-failed = อัปโหลดไม่สำเร็จ: { $reason }
error-upload-no-files = ไม่มีไฟล์ในการอัปโหลด
error-analyze-nothing-to-do = ไม่มีอะไรต้องทำ: rules=false และ LLM ปิดอยู่
error-too-many-assets = แอสเซ็ตในคำขอเดียวมากเกินไป
error-change-not-found = ไม่มีการเปลี่ยนแปลงนั้นในประวัติของแอสเซ็ตนี้
error-bulk-by-look = การค้นหาตามลักษณะไม่สามารถแก้ไขทั้งหมดพร้อมกันได้ ให้เลือกแอสเซ็ตแทน
error-too-many-items = รายการในคำขอเดียวมากเกินไป
error-not-a-hash = ไม่ใช่แฮช blake3 แบบเลขฐานสิบหก
error-merge-into-tag = into ต้องเป็นแท็ก
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = "{ $text }" ไม่ใช่แท็ก
error-not-a-character = แอนิเมชันของโครงกระดูกเล่นได้บนตัวละครเท่านั้น
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } อยู่ใต้ { $tag } อยู่แล้ว จึงใส่ { $tag } ไว้ใต้แท็กนั้นไม่ได้
error-nothing-to-merge = ไม่มีอะไรให้รวม
error-selection-size = ระบุ ID แอสเซ็ตตั้งแต่ 1 ถึง { $max } รายการ
error-collection-needs-name = คอลเลกชันต้องมีชื่อ
error-collection-exists = มีคอลเลกชันชื่อ { $name } อยู่แล้ว
error-collection-empty = คอลเลกชัน { $name } ว่างเปล่า
error-conversion-failed = แปลงเป็น { $format } ไม่ได้: { $reason }
error-max-size = max ต้องเป็นกำลังของสองตั้งแต่ 64 ถึง 16384
error-replace-ext = ext ต้องเป็นนามสกุลไฟล์แบบสั้น
error-empty-file = ไฟล์ว่างเปล่า
error-imitation-silent = ไม่มีเสียงใดในการบันทึกที่เด่นกว่าเสียงพื้นหลัง ลองอีกครั้งโดยอยู่ใกล้ไมโครโฟนมากขึ้น
error-replace-duplicate = ไฟล์ที่แปลงแล้วซ้ำกับไฟล์ที่มีอยู่ในแพ็กแล้ว จึงรวมแพ็กเกจเข้ากับไฟล์นั้น
error-ai-address = ที่อยู่ของ AI ควรขึ้นต้นด้วย http:// หรือ https://
error-hosted-key = ใส่คีย์ API ของ { $provider }
error-hosted-address = ที่อยู่ของบริการค้นหาด้วยภาพควรขึ้นต้นด้วย http:// หรือ https://
error-local-llm-action = action ต้องเป็น list, download หรือ delete และสองอย่างหลังต้องระบุโมเดล
error-role = บทบาทต้องเป็น view, edit หรือ admin
error-admin-only = เฉพาะผู้ดูแลระบบเท่านั้นที่ทำสิ่งนี้ได้
error-view-only = บัญชีนี้ดูได้อย่างเดียวแต่เปลี่ยนแปลงอะไรไม่ได้: ผู้ดูแลระบบเปลี่ยนสิทธิ์นี้ได้ที่ การตั้งค่า → บัญชี
error-portal-only = ลงชื่อเข้าใช้พอร์ทัลเพื่อจัดการบัญชีและโทเค็น
error-not-from-portal = คำขอนี้ไม่ได้มาจากพอร์ทัล Hoard
error-account-name = เลือกชื่อที่ยาวไม่เกิน 64 ตัวอักษร
error-password-length = ใช้รหัสผ่านที่ยาวอย่างน้อย { $min } ตัวอักษร
error-too-many-codes = ใส่รหัสผิดหลายครั้งเกินไป: รอ { $seconds } วินาที
error-already-set-up = มีบัญชีอยู่แล้ว: ลงชื่อเข้าใช้
error-too-many-tries = ลองผิดหลายครั้งเกินไป: รอ { $seconds } วินาที
error-directory-unavailable = ไดเรกทอรีตรวจสอบรหัสผ่านของคุณไม่ได้: { $reason }
error-password-elsewhere = รหัสผ่านของคุณเก็บไว้ที่ไดเรกทอรีหรือผู้ให้บริการที่คุณใช้ลงชื่อเข้าใช้
error-current-password = รหัสผ่านปัจจุบันของคุณไม่ถูกต้อง
error-account-source = บัญชีลงชื่อเข้าใช้ด้วย Hoard (internal), ไดเรกทอรี (ldap) หรือ OAuth
error-account-exists = มีบัญชีชื่อนี้อยู่แล้ว
error-own-role = ผู้ดูแลระบบคนอื่นเป็นผู้เปลี่ยนบทบาทของคุณได้
error-remove-self = คุณลบบัญชีของตัวเองขณะที่ลงชื่อเข้าใช้บัญชีนั้นอยู่ไม่ได้
error-token-name = ตั้งชื่อโทเค็นตามสิ่งที่จะใช้งาน ยาวไม่เกิน 80 ตัวอักษร
error-token-role = โทเค็นทำได้ไม่เกินกว่าบัญชีที่สร้างโทเค็นนั้น
error-wrong-setup-code = นี่ไม่ใช่รหัสตั้งค่าที่กำหนดให้เซิร์ฟเวอร์ใน HOARD_SETUP_CODE
error-wrong-password = ชื่อและรหัสผ่านไม่ตรงกัน
error-folder-missing = ไม่พบ { $path } (ดิสก์ที่ยังไม่ได้เมานต์หรือเปล่า?)
error-s3-path = ตำแหน่ง S3 มีรูปแบบเป็น s3://bucket
error-s3-address = ที่อยู่ S3 ต้องขึ้นต้นด้วย http:// หรือ https://
error-s3-header = ส่วนหัว S3 เพิ่มเติมมีรูปแบบเป็น Name=value แต่ “{ $header }” ไม่ใช่รูปแบบนี้
error-keychain = เก็บไว้ในที่เก็บรหัสผ่านของระบบไม่ได้: { $reason }
error-not-built = Hoard ชุดนี้สร้างโดยไม่มี { $feature }
error-openai-auth-unreachable = เชื่อมต่อ auth.openai.com ไม่ได้
error-openai-unexpected = คำตอบจาก OpenAI ไม่เป็นไปตามที่คาดไว้
error-openai-answered = OpenAI ตอบกลับ { $status }: { $reason }
error-chatgpt-id-token-not-jwt = ID token ไม่ใช่ JWT
error-chatgpt-id-token-issuer = ID token มาจาก { $issuer } ไม่ใช่ { $expected }
error-chatgpt-id-token-audience = ID token นี้เป็นของแอปอื่น
error-chatgpt-id-token-expired = ID token หมดอายุแล้ว
error-chatgpt-id-token-nonce = ID token นี้ไม่ใช่ของการลงชื่อเข้าใช้ครั้งนี้
error-chatgpt-no-state = ที่อยู่ไม่มี state ให้คัดลอกที่อยู่ทั้งหมด
error-chatgpt-not-started = การลงชื่อเข้าใช้นี้ไม่ได้เริ่มที่นี่ หรือเริ่มไปนานกว่า 15 นาทีแล้ว เริ่มใหม่อีกครั้งจากการตั้งค่า
error-chatgpt-canceled = การลงชื่อเข้าใช้ถูกยกเลิก
error-chatgpt-refused = ChatGPT ปฏิเสธการลงชื่อเข้าใช้: { $reason }
error-chatgpt-no-code = ที่อยู่ไม่มีรหัสลงชื่อเข้าใช้ ให้คัดลอกที่อยู่ทั้งหมด
error-chatgpt-other-registration = ChatGPT ตอบกลับสำหรับการลงทะเบียนอื่นของเซิร์ฟเวอร์นี้ ให้เริ่มลงชื่อเข้าใช้ใหม่
error-chatgpt-other-registration-desktop = ChatGPT ตอบกลับสำหรับสำเนา Hoard อื่น ให้เริ่มลงชื่อเข้าใช้ใหม่
error-chatgpt-not-registered = ChatGPT ลงทะเบียน Hoard ไม่เสร็จ ให้เริ่มลงชื่อเข้าใช้ใหม่
error-chatgpt-code-used = รหัสลงชื่อเข้าใช้ถูกใช้ไปแล้วหรือหมดอายุแล้ว ให้ลงชื่อเข้าใช้ใหม่
error-chatgpt-refused-code = ChatGPT ปฏิเสธการลงชื่อเข้าใช้ ({ $code })
error-chatgpt-no-id-token = ChatGPT ไม่ได้ส่ง ID token มา
error-chatgpt-plan-not-allowed = ลงชื่อเข้าใช้แล้ว แต่ไม่ได้รับอนุญาตให้ใช้แพ็กเกจ ให้ลงชื่อเข้าใช้ใหม่และอนุญาต หรือใช้คีย์ API แทน
error-chatgpt-problem = ChatGPT: { $problem }
error-chatgpt-not-signed-in = ยังไม่ได้ลงชื่อเข้าใช้ ChatGPT ให้ลงชื่อเข้าใช้จาก การตั้งค่า → การติดแท็กด้วย AI
error-chatgpt-refresh-later = ChatGPT รีเฟรชการลงชื่อเข้าใช้ไม่ได้ในตอนนี้ ({ $code }) จะลองอีกครั้งในภายหลัง
error-chatgpt-signed-out-problem = ChatGPT ลงชื่อออกเซิร์ฟเวอร์นี้แล้ว ({ $code }) ให้ลงชื่อเข้าใช้ใหม่เพื่อติดแท็กด้วยแพ็กเกจของคุณต่อไป
error-chatgpt-signed-out-problem-desktop = ChatGPT ลงชื่อออก Hoard แล้ว ({ $code }) ให้ลงชื่อเข้าใช้ใหม่เพื่อติดแท็กด้วยแพ็กเกจของคุณต่อไป
error-chatgpt-signed-out = ChatGPT ลงชื่อออกเซิร์ฟเวอร์นี้แล้ว ({ $code }) ให้ลงชื่อเข้าใช้ใหม่จาก การตั้งค่า → การติดแท็กด้วย AI
error-chatgpt-signed-out-desktop = ChatGPT ลงชื่อออก Hoard แล้ว ({ $code }) ให้ลงชื่อเข้าใช้ใหม่จาก การตั้งค่า → การติดแท็กด้วย AI
error-openai-api-unreachable = เชื่อมต่อ api.openai.com ไม่ได้
error-openai-broke-off = คำตอบจาก OpenAI ขาดกลางคัน
error-chatgpt-kept-refusing = OpenAI ปฏิเสธการลงชื่อเข้าใช้ ChatGPT ซ้ำ ๆ ให้ลงชื่อเข้าใช้ใหม่จาก การตั้งค่า → การติดแท็กด้วย AI
error-chatgpt-request-failed = คำขอไม่สำเร็จ
error-chatgpt-cut-off = คำตอบถูกตัดกลางคัน ({ $reason })
error-chatgpt-cut-off-no-reason = คำตอบถูกตัดกลางคัน (ไม่ระบุเหตุผล)
error-openai-ended-early = คำตอบจาก OpenAI จบก่อนจะครบถ้วน
error-chatgpt-usage-limit = โควตาการใช้งานแพ็กเกจ ChatGPT ของคุณสำหรับ Hoard หมดแล้วในตอนนี้ ดูหรือเปลี่ยนขีดจำกัดได้ที่ { $url }
error-chatgpt-not-eligible = บัญชีหรือเวิร์กสเปซ ChatGPT นี้ใช้แพ็กเกจของตนในแอปอื่นไม่ได้ ให้ใช้คีย์ API แทน
error-chatgpt-usage-unavailable = ChatGPT ตรวจสอบการใช้งานแพ็กเกจไม่ได้ในตอนนี้ จะลองอีกครั้งในภายหลัง
error-chatgpt-unsupported = การใช้งานผ่านแพ็กเกจ ChatGPT ไม่รองรับ { $param } กับโมเดลนี้
error-chatgpt-unsupported-images = การใช้งานผ่านแพ็กเกจ ChatGPT ไม่รองรับ { $param } กับโมเดลนี้ ให้เลือกโมเดลที่รับภาพได้ หรือปิดการส่งภาพย่อ
error-chatgpt-unsupported-part = การใช้งานผ่านแพ็กเกจ ChatGPT ไม่รองรับบางส่วนของคำขอกับโมเดลนี้
error-chatgpt-port = port คือพอร์ตที่เบราว์เซอร์ใช้เข้าถึง Hoard
error-chatgpt-paste-address = วางที่อยู่ทั้งหมดที่ขึ้นต้นด้วย http://127.0.0.1
error-too-many-hashes = ครั้งละไม่เกิน { $max } แฮช
error-credits-pack = แพ็ก { $key }
error-credits-collection = คอลเลกชัน { $key }
error-format-too-long = รูปแบบยาวเกินไป
error-not-started = เซิร์ฟเวอร์ยังไม่เริ่มทำงาน
error-not-started-desktop = Hoard ยังไม่เริ่มทำงาน
error-not-format-file = ไม่ใช่ไฟล์ของรูปแบบ: { $name }
error-format-name = ชื่อต้องมีตัวอักษรหรือตัวเลขตั้งแต่ A ถึง Z หรือ 0 ถึง 9
error-format-miss-name = ไฟล์ของรูปแบบนี้ไม่รับ { $file }
error-format-miss-store = รูปแบบนี้ไม่รับ { $file } สำหรับไฟล์ของร้านค้า (ดู require)
error-format-miss-title = รูปแบบนี้หาชื่อเรื่องใน { $file } ไม่พบ
error-cant-download-as = ไฟล์ .{ $ext } ดาวน์โหลดเป็น { $format } ไม่ได้
error-only-images-resize = ปรับขนาดได้เฉพาะภาพเท่านั้น
error-https-no-certificate = วางหรืออัปโหลดใบรับรองและคีย์ของใบรับรองก่อน
error-https-no-cert-found = ไม่พบใบรับรอง: วางข้อความที่ขึ้นต้นด้วย -----BEGIN CERTIFICATE-----
error-https-no-key-found = ไม่พบคีย์ส่วนตัว: วางข้อความที่ขึ้นต้นด้วย -----BEGIN PRIVATE KEY-----
error-https-several-keys = มีคีย์ส่วนตัวมากกว่าหนึ่งคีย์: วางเฉพาะคีย์ที่คู่กับใบรับรอง
error-https-key-password = คีย์ถูกป้องกันด้วยรหัสผ่าน: ให้ส่งคีย์ที่ไม่มีรหัสผ่าน (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = ใบรับรองไม่ใช่ PEM ที่ถูกต้อง
error-https-key-not-pem = คีย์ส่วนตัวไม่ใช่ PEM ที่ถูกต้อง
error-https-key-mismatch = คีย์ไม่ได้คู่กับใบรับรอง หากคุณให้ใบรับรองแบบเชน ให้ใส่ใบรับรองของเซิร์ฟเวอร์คุณไว้เป็นอันแรก
error-https-unusable = ใช้ใบรับรองหรือคีย์นี้ไม่ได้
error-https-expired = ใบรับรองนี้หมดอายุเมื่อ { $date }
error-https-not-yet-valid = ใบรับรองนี้จะใช้ได้ตั้งแต่ { $date }
error-https-unreadable = นี่ไม่ใช่ใบรับรองที่ Hoard อ่านได้
error-https-bad-name = “{ $name }” ไม่ใช่ที่อยู่ที่ใบรับรองระบุได้
error-https-test-timed-out = การเชื่อมต่อทดสอบหมดเวลา
error-https-test-failed = การเชื่อมต่อทดสอบไม่สำเร็จ
error-https-no-ca = Hoard ยังไม่ได้สร้างผู้ออกใบรับรอง (CA) ของตัวเอง
error-https-unavailable = เซิร์ฟเวอร์นี้ใช้ HTTPS ไม่ได้
error-https-certificate-mode = ใบรับรองใช้คู่กับตัวเลือก “ใบรับรองของฉัน”
error-https-test = ใบรับรองใช้ไม่ได้ในการเชื่อมต่อทดสอบ: { $reason }
error-nothing-to-retry = ไม่มีอะไรที่ลองใหม่ได้
error-ai-unreachable = เชื่อมต่อ { $url } ไม่ได้
error-ai-key-refused = { $url } ปฏิเสธคีย์ API ({ $status })
error-ai-answered = { $url } ตอบกลับ { $status }: { $reason }
error-ai-not-json = { $url }/models ไม่ได้ตอบกลับเป็น JSON
error-model-server-refused = เซิร์ฟเวอร์โมเดล: { $reason }
error-no-model-server = ไม่มีเซิร์ฟเวอร์โมเดลตอบกลับที่ { $url }
error-not-llama-router = { $url } ไม่ใช่เซิร์ฟเวอร์ llama.cpp ในโหมด router จึงดาวน์โหลดโมเดลไม่ได้
error-model-download = ดาวน์โหลด { $model } ไม่ได้: { $reason }
error-model-delete = ลบ { $model } ไม่ได้: { $reason }
error-unknown-kind = ไม่รู้จักประเภทแอสเซ็ต { $kind }
error-watched-is-folder = ตำแหน่งที่เฝ้าดูคือโฟลเดอร์บนเซิร์ฟเวอร์นี้
error-cant-reach = เข้าถึง { $path } ไม่ได้: { $reason }
error-folder-full-path = ระบุพาธเต็มของโฟลเดอร์
error-cant-create = สร้าง { $path } ไม่ได้
error-cant-write = เขียนลง { $path } ไม่ได้
error-cant-read = อ่าน { $path } ไม่ได้
error-overlaps-location = ซ้อนทับกับตำแหน่ง { $name } ({ $path })
error-overlaps-data = ซ้อนทับกับโฟลเดอร์ข้อมูลของ Hoard ({ $path })
error-overlaps-cache = ซ้อนทับกับโฟลเดอร์แคชของ Hoard ({ $path })
error-not-watched = { $name } ไม่ใช่โฟลเดอร์ที่เฝ้าดู
error-watched-no-kinds = โฟลเดอร์ที่เฝ้าดูไม่รับไฟล์ใหม่ จึงไม่มีประเภทให้เปลี่ยน
error-keep-catch-all = ให้ตั้งตำแหน่งอื่นเป็นที่รับทุกอย่างแทน
error-location-holds-files = { $name } ยังมีไฟล์อยู่: ย้ายไฟล์ไปยังตำแหน่งอื่นก่อน (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = เข้าถึง { $name } ไม่ได้ Hoard จึงบอกไม่ได้ว่ายังมีไฟล์อยู่หรือไม่
error-relocate-running = กำลังย้ายไฟล์ระหว่างตำแหน่งอยู่แล้ว ลองอีกครั้งเมื่อเสร็จ
error-watched-not-moved = { $name } เป็นโฟลเดอร์ที่เฝ้าดู: Hoard ไม่ย้ายไฟล์ในโฟลเดอร์นี้
error-no-other-location = ไม่มีตำแหน่งคลังอื่นให้ย้ายไฟล์ไป: เพิ่มตำแหน่งก่อน
error-full-path = ระบุพาธเต็ม
error-pick-folder = เลือกโฟลเดอร์
error-task-running = “{ $task }” กำลังทำงานอยู่แล้ว
error-job-cant-stop = “{ $job }” หยุดไม่ได้เมื่อเริ่มแล้ว
error-job-not-found = งานนั้นไม่ได้รอคิวหรือกำลังทำงานอยู่แล้ว
error-task-cant-stop = “{ $task }” หยุดกลางคันไม่ได้
error-task-not-running = “{ $task }” ไม่ได้กำลังทำงาน
error-token-account-gone = บัญชีของโทเค็นนี้ไม่มีอยู่แล้ว จึงไม่มีรายการโปรดหรือการค้นหา
error-favorites-too-many = ครั้งละไม่เกิน 10,000 รายการ
error-saved-search-name = การค้นหาที่บันทึกไว้ต้องมีชื่อยาวไม่เกิน 80 ตัวอักษร
error-saved-search-too-long = การค้นหานี้ยาวเกินกว่าจะบันทึกได้
error-hoard-seen = ขุมทรัพย์บันทึกแอสเซ็ตได้สูงสุด 32 ชนิด แต่ละชนิดตั้งชื่อด้วยตัวพิมพ์เล็ก
error-news-seen = มีอะไรใหม่บันทึกสิ่งที่เห็นแล้วได้สูงสุด 256 รายการ แต่ละรายการตั้งชื่อด้วยตัวพิมพ์เล็ก ตัวเลข และขีดกลาง
error-texture-pattern-map = รูปแบบชื่อเท็กซ์เจอร์ต้องมี { "{" }map{ "}" } ไม่เช่นนั้นทุกแมปของแมทีเรียลจะได้ชื่อเดียวกัน
error-texture-pattern-fields = รูปแบบชื่อเท็กซ์เจอร์ใช้ { "{" }name{ "}" }, { "{" }map{ "}" } และ { "{" }size{ "}" } ได้
error-texture-name-chars = ชื่อเท็กซ์เจอร์ต้องไม่มี < > : " / \ | ? หรือ *
error-texture-map-type = ไม่มีประเภทแมปชื่อ { $role }
server-role-base-color = สี
server-role-normal = นอร์มอล
server-role-normal-dx = นอร์มอล (DirectX)
server-role-roughness = ความหยาบ
server-role-gloss = ความเงา
server-role-metalness = ความเป็นโลหะ
server-role-ao = แอมเบียนต์อักคลูชัน
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = ความสูง
server-role-bump = บัมป์
server-role-cavity = คาวิตี
server-role-specular = สเปคคิวลาร์
server-role-opacity = ความทึบ
server-role-emissive = การเปล่งแสง
server-role-translucency = ความโปร่งแสง
error-texture-suffix-missing = กำหนดคำต่อท้ายให้แมป { $map }
error-texture-suffix-chars = คำต่อท้ายของแมป { $map } ต้องไม่มี < > : " / \ | ? หรือ *
error-texture-ending-missing = ท้ายชื่อที่คุณกำหนดเองแต่ละรายการต้องระบุท้ายชื่อที่จะค้นหา
error-texture-ending-chars = ท้ายชื่อที่คุณกำหนดเองต้องไม่มี < > : " / \ | ? หรือ *
error-texture-ending-twice = { $ending } อยู่ในท้ายชื่อของคุณสองครั้ง
error-oauth-no-client-id = ลิงก์นี้ไม่มี client_id
error-oauth-unknown-app = แอปนี้ไม่ได้ลงทะเบียนไว้ที่นี่ (แล้ว) ให้เริ่มเชื่อมต่อใหม่จากแอป
error-oauth-redirect = แอปขอให้ส่งกลับไปยังที่อยู่ที่ไม่ได้ลงทะเบียนไว้
error-oauth-response-type = Hoard ให้เฉพาะรหัสลงชื่อเข้าใช้เท่านั้น (response_type=code)
error-oauth-pkce = แอปต้องใช้ PKCE กับ S256 เพื่อลงชื่อเข้าใช้ Hoard
error-light-preview-images = มีเพียงภาพที่มีตัวอย่างแบบเบา
error-no-light-preview = ไม่มีตัวอย่างแบบเบา: { $reason }
error-no-frame-sheet = ไม่มีแผ่นเฟรม: { $reason }
error-thumbnail-source = ไม่รู้จักแหล่งภาพย่อ: { $source }
error-thumbnail-too-big = ภาพนั้นใหญ่เกินไปสำหรับภาพย่อ
error-thumbnail-models-only = มีเพียงโมเดลที่ใช้ภาพย่อที่เบราว์เซอร์วาดได้
error-not-a-picture = ไม่ใช่ภาพที่ Hoard อ่านได้: { $reason }
error-project-name = โปรเจกต์ต้องมีชื่อยาวไม่เกิน 200 ตัวอักษร
error-project-too-big = โปรเจกต์หนึ่งมีแอสเซ็ตได้ไม่เกินหนึ่งล้านรายการ
error-project-too-many-at-once = ครั้งละไม่เกิน 100,000 รายการ
error-server-name = เลือกชื่อที่ยาวไม่เกิน { $max } ตัวอักษร ในบรรทัดเดียว
error-tool-setting = { $tool }: เลือก ปิด, อัตโนมัติ หรือระบุพาธ
error-ai-model = เลือกโมเดลที่บริการ AI ควรใช้
error-public-address = ที่อยู่สาธารณะควรขึ้นต้นด้วย http:// หรือ https://
error-update-hook-address = ที่อยู่ที่จะเรียกเมื่อมีการอัปเดตควรขึ้นต้นด้วย http:// หรือ https://
error-update-hook-auth = ส่วนหัว Authorization ต้องไม่มีการขึ้นบรรทัดใหม่หรืออักขระควบคุมอื่น ๆ
error-no-soundfont = ไม่มี SoundFont ที่ { $path } บนเซิร์ฟเวอร์
error-no-soundfont-desktop = ไม่มี SoundFont ที่ { $path }
error-soundfont-bad = { $name } ไม่ใช่ SoundFont ที่ MIDI ใช้เล่นได้: { $reason }
error-workers = ตัวทำงานเบื้องหลัง: ตั้งแต่ 1 ถึง { $max }
error-ai-batch = แอสเซ็ตต่อคำขอ AI: ตั้งแต่ 1 ถึง 200
error-conversion-cache = การดาวน์โหลดที่แปลงแล้ว: อย่างน้อย 100 MB
error-motion-fps = ภาพขนาดย่อแบบเคลื่อนไหวเล่นที่ 12, 24 หรือ 30 เฟรมต่อวินาที
error-thumb-size = ภาพย่อมีขนาด 128 หรือ 256 พิกเซล
error-backup-keep = การสำรองข้อมูลตามกำหนดเวลา: เก็บได้ 1 ถึง { $max } ชุด
error-backup-copy-to = การสำรองข้อมูลตามกำหนดเวลา: { $path } ไม่ใช่โฟลเดอร์บนเซิร์ฟเวอร์ (ระบุพาธเต็ม)
error-conversion-keep-days = กฎการแปลง: เก็บต้นฉบับไว้ได้ 1 ถึง { $max } วัน
error-conversion-target = กฎการแปลง: Hoard แปลงไฟล์เป็น { $format } ไม่ได้
error-conversion-no-source = กฎการแปลง: กฎแต่ละข้อต้องมีรูปแบบที่จะแปลงอย่างน้อยหนึ่งรูปแบบ
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = กฎการแปลง: แปลงไฟล์ .{ $ext } เป็น { $format } ไม่ได้
error-conversion-twice = กฎการแปลง: .{ $ext } อยู่ในกฎสองข้อ
error-conversion-cant-undo = ไฟล์นี้ไม่ได้ถูกแปลงโดยกฎ หรือไฟล์ต้นฉบับถูกลบไปแล้ว
error-conversion-path-taken = ตอนนี้มีไฟล์อื่นอยู่ที่ { $path } จึงนำต้นฉบับกลับไปไว้ที่นั่นไม่ได้
error-limits-zero = ขีดจำกัดต้องไม่เป็นศูนย์
error-settings-object = ต้องเป็นออบเจกต์ JSON ของการตั้งค่า
error-bad-setting = การตั้งค่าไม่ถูกต้อง: { $reason }
error-auth-default-role = บทบาทเริ่มต้นต้องเป็น view, edit หรือ admin
error-auth-session-hours = เซสชันมีอายุตั้งแต่ 1 ชั่วโมงถึงหนึ่งปี
error-auth-settings = การตั้งค่าการยืนยันตัวตนไม่ถูกต้อง: { $reason }
error-ldap-settings = การตั้งค่า LDAP ไม่ถูกต้อง: { $reason }
error-ldap-no-server = เพิ่มที่อยู่ของเซิร์ฟเวอร์ไดเรกทอรี
error-ldap-base-dn = ระบุว่ารายการของผู้ใช้อยู่ที่ใด (base DN)
error-ldap-username-attribute = ระบุว่าแอตทริบิวต์ใดเก็บชื่อที่ผู้ใช้ใช้ลงชื่อเข้าใช้
error-ldap-bind-dn = ระบุบัญชีที่ใช้ค้นหาผู้ใช้ หรือค้นหาแบบไม่ระบุตัวตน
error-ldap-ca-cert = ผู้ออกใบรับรองไม่ใช่ใบรับรอง PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = ที่อยู่ของเซิร์ฟเวอร์ไดเรกทอรีว่างเปล่า
error-ldap-server-form = { $server }: ที่อยู่ของเซิร์ฟเวอร์ไดเรกทอรีมีรูปแบบเป็น host:port, ldap://… หรือ ldaps://…
error-ldap-not-starttls = { $server }: นั่นคือ LDAPS ไม่ใช่ StartTLS
error-ldap-no-certificate = ไม่มีใบรับรองอยู่ในนั้น
error-ldap-password-again = ระบุรหัสผ่านของบัญชีที่ใช้ค้นหาผู้ใช้อีกครั้ง: รหัสผ่านที่บันทึกไว้จะส่งไปยังเซิร์ฟเวอร์ที่กำหนดไว้ตอนบันทึกเท่านั้น
error-ldap-unreachable = เชื่อมต่อเซิร์ฟเวอร์ไดเรกทอรีไม่ได้เลย ({ $reasons })
error-ldap-no-anonymous = ไดเรกทอรีไม่อนุญาตให้ค้นหาผู้ใช้แบบไม่ระบุตัวตน ({ $reason })
error-ldap-bind-failed = บัญชีที่ใช้ค้นหาผู้ใช้ลงชื่อเข้าใช้ไม่ได้ ({ $reason })
error-ldap-search = กำลังค้นหา { $name } ภายใต้ { $base }
error-ldap-group-search = กำลังค้นหากลุ่มของ { $name } ภายใต้ { $base }
error-ldap-several = มี { $count } รายการในไดเรกทอรีที่ตรงกับ { $name }: ให้จำกัดตัวกรองผู้ใช้ให้แคบลง
error-ldap-password-check = ไดเรกทอรีไม่ยอมตรวจสอบรหัสผ่านของ { $name } ({ $reason })
error-ldap-wrong-password = ไดเรกทอรีไม่รู้จัก { $name } ที่ใช้รหัสผ่านนี้
error-oauth-client-id = ระบุ client ID ที่ผู้ให้บริการสร้างให้ Hoard
error-oauth-addresses = ที่อยู่ authorization และ token ควรขึ้นต้นด้วย https://
error-oauth-userinfo-address = ที่อยู่ user info ควรขึ้นต้นด้วย https://
error-oauth-username-claim = ระบุว่า claim ใดเก็บชื่อของผู้ใช้
error-oauth-secret-again = ระบุ client secret อีกครั้ง: ค่าที่บันทึกไว้จะส่งไปยังที่อยู่ token ที่กำหนดไว้ตอนบันทึกเท่านั้น
error-oauth-issuer = issuer ควรขึ้นต้นด้วย https://
error-oauth-discovery-unreachable = เชื่อมต่อ { $url } ไม่ได้: { $reason }
error-oauth-discovery-status = { $url } ตอบกลับ { $status }
error-oauth-discovery-bad = { $url } ไม่ใช่เอกสาร discovery
error-oauth-discovery-incomplete = { $url } ไม่ได้ระบุที่อยู่ authorization และ token
error-signin-no-group = { $name } ไม่ได้อยู่ในกลุ่มที่ใช้ Hoard ได้
error-signin-other-source = มีบัญชี Hoard ชื่อ { $name } ที่ลงชื่อเข้าใช้ด้วยวิธีอื่นอยู่แล้ว
error-signin-other-subject = มีบัญชี Hoard ชื่อ { $name } ที่เป็นของบุคคลอื่นในผู้ให้บริการอยู่แล้ว
error-signin-no-account = { $name } ยังไม่มีบัญชี Hoard: ผู้ดูแลระบบเพิ่มบัญชีได้ที่ การตั้งค่า → บัญชี
error-signin-bad-name = ชื่อนี้ใช้เป็นบัญชี Hoard ไม่ได้ (ยาวไม่เกิน 64 ตัวอักษร)
error-oauth-off = ยังไม่ได้เปิดการลงชื่อเข้าใช้ด้วย OAuth
error-oauth-bad-authorization-address = ที่อยู่ authorization ไม่ถูกต้อง
error-oauth-refused = ผู้ให้บริการไม่ได้ให้คุณลงชื่อเข้าใช้: { $reason }
error-oauth-no-code = ผู้ให้บริการส่งคุณกลับมาโดยไม่มีการลงชื่อเข้าใช้
error-oauth-expired = การลงชื่อเข้าใช้นี้ใช้เวลานานเกินไปหรือเริ่มจากที่อื่น: ลองอีกครั้ง
error-oauth-no-claim = ผู้ให้บริการไม่ได้บอกว่าคุณคือใคร (ไม่มี claim { $claim })
error-oauth-no-sub = ผู้ให้บริการไม่ได้บอกว่าคุณคือใคร (ไม่มี claim sub หรือ id)
error-oauth-token-unreachable = เชื่อมต่อที่อยู่ token ของผู้ให้บริการไม่ได้
error-oauth-token-refused = ผู้ให้บริการไม่ยอมดำเนินการลงชื่อเข้าใช้ให้เสร็จ: { $reason }
error-oauth-no-access-token = ผู้ให้บริการไม่ได้ส่งโทเค็นการเข้าถึงมา
error-oauth-userinfo-unreachable = เชื่อมต่อที่อยู่ user info ของผู้ให้บริการไม่ได้
error-oauth-userinfo-refused = ผู้ให้บริการไม่ยอมบอกว่าใครลงชื่อเข้าใช้ ({ $status })
error-oauth-userinfo-not-json = user info ของผู้ให้บริการไม่ใช่ JSON
error-format-no-name = name ว่างเปล่า
error-format-no-files = files ว่างเปล่า: ระบุว่ารูปแบบนี้อ่านไฟล์ใด เช่น ["*.json"]
error-format-empty-selector = ว่างเปล่า
error-format-unopened-bracket = มีวงเล็บปิดโดยไม่มีวงเล็บเปิด
error-format-unclosed-bracket = มีวงเล็บที่ไม่ได้ปิด
error-format-empty-step-dots = มีขั้นที่ว่างเปล่า (จุดสองจุดติดกัน หรือมีจุดที่ปลายด้านใดด้านหนึ่ง)
error-format-unclosed-except = !( ไม่ได้ปิด
error-format-unclosed-filter = [ ไม่ได้ปิด
error-format-after-filters = มี { $rest } ต่อจากคีย์และตัวกรองของ { $step }
error-format-except-needs-glob = !(…) ต้องอยู่หลังคีย์ที่มี * อยู่
error-format-except-not-glob = !(…) ต้องอยู่หลังคีย์ที่มี * อยู่ ไม่ใช่ { $key }
error-format-empty-step = มีขั้นที่ว่างเปล่า
error-format-not-a-filter = [{ $filter }] ไม่ใช่ตัวกรอง: ใช้ [], [key=value|other], [array], [string], [number], [bool] หรือ [object]
error-format-filter-needs-value = [{ $filter }] ต้องมีคีย์และค่า
error-store-search-empty = ไม่มีอะไรให้ค้นหา ให้พิมพ์ชื่อแพ็ก
error-update-hook-not-set = ตั้งที่อยู่ที่จะเรียกก่อน
error-upload-stalled = การอัปโหลด { $path } หยุดชะงัก ให้ส่งส่วนนั้นอีกครั้ง
error-upload-busy = กำลังเขียน { $path } อยู่แล้ว
error-upload-incomplete = มี { $count } ไฟล์ที่ไม่ครบ เช่น { $files }
error-upload-no-files-listed = ไม่มีไฟล์
error-upload-not-part = { $path } ไม่ได้เป็นส่วนหนึ่งของการอัปโหลด { $id } หรือการอัปโหลดปิดไปแล้ว
error-bad-path = พาธไม่ถูกต้อง
error-upload-part-offset = { $offset } ไม่ใช่ตำแหน่งเริ่มต้นของส่วนใดส่วนหนึ่งของ { $path }
error-upload-interrupted = การอัปโหลดถูกขัดจังหวะ: { $reason }
error-upload-part-long = ส่วนที่ { $part } ของ { $path } ยาวเกิน { $len } ไบต์
error-upload-part-short = ส่วนที่ { $part } ของ { $path }: ได้รับ { $got } ไบต์ แต่คาดว่าจะได้ { $len }
error-upload-file-long = { $path } ใหญ่กว่าที่แจ้งไว้
error-upload-file-short = { $path }: ได้รับ { $got } ไบต์ แต่คาดว่าจะได้ { $len }
error-upload-complete = การอัปโหลดเสร็จสมบูรณ์แล้ว
error-already-imported = นำเข้าแล้วในชื่อ “{ $name }”

## ข้อความของเซิร์ฟเวอร์

server-store-place-readme = README
server-store-place-folders = โฟลเดอร์
server-store-why-linked = มีลิงก์อยู่ใน { $place } ของแพ็ก
server-store-why-same-title = ชื่อเดียวกับใน { $place } ของแพ็ก
server-store-why-by = โดย { $author } ซึ่งระบุชื่อไว้ใน { $place } ของแพ็ก
server-store-why-meshes = จำนวนเมชใกล้เคียงกับแพ็ก ({ $count })
server-store-why-textures = จำนวนเท็กซ์เจอร์ใกล้เคียงกับแพ็ก ({ $count })
server-chatgpt-revoke-failed = ลงชื่อออกที่นี่แล้ว แต่แจ้ง OpenAI ไม่ได้ เพื่อความแน่ใจ ให้ยกเลิกการเชื่อมต่อ Hoard ใน ChatGPT ที่ Settings → Security and login → Sign in with ChatGPT
server-chatgpt-signed-in-title = ลงชื่อเข้าใช้ ChatGPT แล้ว
server-chatgpt-signed-in = Hoard ลงชื่อเข้าใช้แล้ว ตอนนี้การติดแท็กด้วย AI ใช้แพ็กเกจ ChatGPT ของคุณได้แล้ว
server-chatgpt-signed-in-as = Hoard ลงชื่อเข้าใช้ในชื่อ { $email } แล้ว ตอนนี้การติดแท็กด้วย AI ใช้แพ็กเกจ ChatGPT ของคุณได้แล้ว
server-chatgpt-not-finished-title = การลงชื่อเข้าใช้ยังไม่เสร็จ
server-chatgpt-other-computer = หากคุณเริ่มลงชื่อเข้าใช้จาก Hoard บนคอมพิวเตอร์เครื่องอื่น ให้คัดลอกที่อยู่ของหน้านี้จากแถบที่อยู่ แล้ววางลงในการตั้งค่าของ Hoard บนเครื่องนั้น
server-back-to-settings = กลับไปที่การตั้งค่า
server-issues-pack = แพ็กที่เพิ่มไม่ได้
server-issues-pack-blurb = ลบแพ็กแล้วเพิ่มใหม่ หรือเปิดดูว่ามีอะไรอยู่บ้าง
server-issues-damaged = ไฟล์ที่จัดเก็บไว้ซึ่งเสียหายหรือหายไป
server-issues-damaged-blurb = พบโดย “ตรวจสอบไฟล์ว่าเสียหายหรือไม่” กู้คืนไฟล์จากข้อมูลสำรองหรือเพิ่มแพ็กของไฟล์ใหม่ แล้วเรียกใช้ “ตรวจสอบไฟล์ว่าเสียหายหรือไม่” อีกครั้ง
server-issues-folder = โฟลเดอร์ที่เฝ้าดูซึ่งอ่านไม่ได้
server-issues-folder-blurb = แพ็กในโฟลเดอร์จะคงไว้ตามเดิมจนกว่าจะอ่านโฟลเดอร์ได้อีกครั้ง
server-issues-missing = แพ็กที่เฝ้าดูซึ่งโฟลเดอร์หายไป
server-issues-missing-blurb = แต่ละแพ็กจะเก็บไว้ 3 วันเผื่อว่าโฟลเดอร์แค่ออฟไลน์อยู่ จากนั้นจะถูกนำออก
server-issues-job = งานเบื้องหลังที่ล้มเหลว
server-issues-job-blurb = การลองใหม่จะเรียกใช้งานนั้นอีกครั้งตั้งแต่ต้น
server-issues-file = ไฟล์ที่เปิดไม่ได้
server-issues-file-blurb = Hoard เปิดหรือจัดเก็บไฟล์เหล่านี้ไม่ได้ แพ็กที่เฝ้าดูจะลองอีกครั้งในการสแกนครั้งถัดไป ส่วนไฟล์ที่อยู่ในคลังแล้วจะคงสิ่งที่มีอยู่เดิมไว้
server-issues-unpack = ไฟล์บีบอัดที่แตกไฟล์ไม่ได้
server-issues-unpack-blurb = แต่ละไฟล์จะเก็บไว้ในแพ็กเป็นไฟล์ธรรมดา จึงไม่มีสิ่งใดข้างในอยู่ในคลัง
server-issues-read = ไฟล์ที่ Hoard อ่านความหมายไม่ออก
server-issues-read-blurb = ไฟล์เหล่านี้อยู่ในคลังแล้ว แต่ไม่มีข้อมูล แท็ก หรือภาพย่อที่ได้จากการอ่านไฟล์
server-issues-render = โมเดลที่ตัวเรนเดอร์ภาพย่อทำงานล้มเหลว
server-issues-render-blurb = โมเดลเหล่านี้ไม่มีภาพย่อ การเปิดในตัวแสดงผลอาจยังใช้ได้ และบันทึกภาพย่อได้
server-issues-convert = โมเดลที่ Blender แปลงไม่ได้
server-issues-convert-blurb = โมเดลเหล่านี้ไม่มีสำเนา GLB จึงแสดงแบบ 3 มิติไม่ได้
server-issues-unreal = แพ็กเกจ Unreal ที่แปลงไม่ได้
server-issues-unreal-blurb = หน้า Unreal ของแพ็กมีแพ็กเกจที่เหลือของแพ็กนั้น
server-issues-reformat = ไฟล์ที่กฎการแปลงแปลงไม่ได้
server-issues-reformat-blurb = ไฟล์เหล่านี้ถูกปล่อยไว้ตามเดิม
server-issues-materials = แพ็กที่ระบุแมทีเรียลไม่ได้
server-issues-materials-blurb = เท็กซ์เจอร์ทั้งหมดยังอยู่ครบ เพียงแต่ไม่ได้จัดกลุ่มเป็นแมทีเรียล
server-issues-material-thumb = แมทีเรียลที่ไม่มีภาพย่อ
server-issues-material-thumb-blurb = พบแมทีเรียลแล้ว แต่วาดตัวอย่างไม่ได้
server-issues-store-files = ไฟล์ร้านค้าที่อ่านไม่ได้
server-issues-store-files-blurb = ไฟล์จากร้านค้าที่แพ็กมาจาก เช่น JSON ของ Megascans ซึ่งใช้ตั้งชื่อและติดแท็กแพ็ก
server-issues-scan-again = สแกนอีกครั้ง
server-issues-retry = ลองใหม่
server-issues-reanalyze = วิเคราะห์แพ็กใหม่
server-issues-convert-again = แปลงอีกครั้ง
server-issues-look-again = ค้นหาอีกครั้ง
server-issues-read-again = อ่านอีกครั้ง
server-issues-blob-missing = ไฟล์ที่จัดเก็บไว้หายไปแล้ว
server-issues-blob-damaged = ไฟล์ที่จัดเก็บไว้ไม่ตรงกับที่บันทึกไว้: ไฟล์เสียหาย
server-issues-folder-gone = ไม่พบโฟลเดอร์ “{ $folder }”
server-job-a-pack = แพ็กหนึ่ง
server-job-ingest = กำลังนำเข้า { $pack }
server-job-retag = กำลังติดแท็ก { $pack } ใหม่
server-job-llm-tag = กำลังติดแท็ก { $pack } ด้วย AI
server-job-convert = กำลังแปลง { $pack } ด้วย Blender
server-job-unreal = กำลังแปลงแพ็กเกจ Unreal ใน { $pack }
server-job-reformat = กำลังแปลงไฟล์ใน { $pack } ตามกฎการแปลง
server-job-scan = กำลังสแกนโฟลเดอร์ที่เฝ้าดู
server-job-relocate = กำลังย้ายไฟล์ระหว่างตำแหน่งคลัง
server-job-looks = กำลังวิเคราะห์สีและลักษณะ
server-job-sounds = กำลังฟังเสียง
server-job-embed = กำลังเรียนรู้ว่าภาพแสดงอะไร
server-job-hear = กำลังเรียนรู้ว่าเสียงฟังดูเป็นอย่างไร
server-job-materials = กำลังค้นหาแมทีเรียลใน { $pack }
server-job-read-files = กำลังอ่านไฟล์ของ { $pack }
server-job-store-lookup = กำลังค้นหาหน้าร้านค้า
server-maintenance-backup = สำรองฐานข้อมูล
server-maintenance-backup-blurb = สำเนาที่สอดคล้องกันของฐานข้อมูล (แพ็ก แท็ก โน้ต คอลเลกชัน และการตั้งค่า) ในโฟลเดอร์ backups ที่อยู่ข้างกัน ไฟล์ในคลังไม่รวมอยู่ด้วย: ไฟล์เหล่านั้นไม่เปลี่ยนแปลงเลยหลังจัดเก็บ ดังนั้นการซิงก์ไฟล์แบบใดก็สำรองไฟล์เหล่านั้นได้
server-maintenance-scrub = ตรวจสอบไฟล์ว่าเสียหายหรือไม่
server-maintenance-scrub-blurb = อ่านไฟล์ที่จัดเก็บไว้ทุกไฟล์และตรวจสอบกับลายนิ้วมือของไฟล์ เพื่อจับไฟล์ที่เสื่อมสภาพหรือหายไปจากดิสก์หรือ NAS ใช้เวลาสักพักสำหรับคลังขนาดใหญ่
server-maintenance-reanalyze = วิเคราะห์ทุกแพ็กใหม่
server-maintenance-reanalyze-blurb = อ่านทุกไฟล์อีกครั้งด้วย Hoard เวอร์ชันนี้: สร้างภาพย่อ ตัวอย่าง รายละเอียด และแท็กตามกฎใหม่ทั้งหมด การแก้ไขของคุณเองยังคงอยู่ และจะไม่ถาม AI อีก
server-maintenance-clear-cache = ล้างไฟล์แปลงที่แคชไว้
server-maintenance-clear-cache-blurb = ลบไฟล์ที่เก็บไว้จากการแปลงด้วย “ดาวน์โหลดเป็น” (จะสร้างใหม่เมื่อมีการขอครั้งถัดไป) และสิ่งที่งานที่ถูกขัดจังหวะทิ้งไว้
server-maintenance-gc = ลบไฟล์ที่ไม่ได้ใช้
server-maintenance-gc-blurb = ลบไฟล์ที่จัดเก็บไว้ซึ่งไม่มีสิ่งใดอ้างถึงแล้ว เช่น ไฟล์ของแพ็กที่ถูกลบถาวร
server-update-no-address = ยังไม่ได้ตั้งที่อยู่ที่จะเรียก
server-update-no-answer = ไม่มีการตอบกลับภายใน 30 วินาที

## การตั้งค่า: รูปลักษณ์

appearance-language = ภาษา
appearance-language-auto = ตามเบราว์เซอร์ ({ $name })
appearance-scheme = สว่างหรือมืด
appearance-scheme-system = ระบบ
appearance-size-section = ข้อความและระยะห่าง
appearance-text-size = ขนาดข้อความ
appearance-text-small = เล็ก
appearance-text-normal = ปกติ
appearance-text-large = ใหญ่
appearance-density = ระยะห่าง
appearance-density-comfortable = โปร่งสบาย
appearance-density-compact = กะทัดรัด
appearance-tiles = ไทล์
appearance-tiles-what = สิ่งที่แต่ละไทล์ในคลังแสดงนอกจากภาพ
appearance-tile-names = ชื่อ
appearance-tile-details = ชนิดและขนาด (พิกเซล ความยาว สามเหลี่ยม)
appearance-tile-badges = ชนิดไฟล์บนภาพ
appearance-tile-motion = เล่นแอนิเมชันขณะที่ตัวชี้อยู่เหนือภาพ
appearance-tile-motion-always = เล่นแอนิเมชันตลอดเวลา ไม่ใช่แค่ตอนที่ตัวชี้อยู่เหนือภาพ
appearance-top-bar = แถบด้านบน
appearance-top-bar-what = ซ่อนสิ่งที่คุณไม่ได้ใช้ / ยังคงไปที่ช่องค้นหา และ { $keys } ยังคงเปิดแผงคำสั่ง
appearance-top-activity = งานเบื้องหลัง
appearance-top-theme = สวิตช์สว่างและมืด
appearance-top-mic = ไมโครโฟนในช่องค้นหา สำหรับค้นหาเสียงด้วยการทำเสียงนั้น
appearance-top-slash = คำใบ้ / ในช่องค้นหา
appearance-top-palette = ปุ่มแผงคำสั่ง ({ $keys })
appearance-news = มีอะไรใหม่
appearance-news-what = สิ่งใหม่ใน Hoard จะมีจุดสีทองระยิบระยับจนกว่าคุณจะได้เห็น และแถบด้านบนจะรวบรวมไว้ในที่เดียว สิ่งที่คุณเห็นแล้วจะผูกกับบัญชีของคุณ
appearance-news-point-out = ชี้สิ่งใหม่ให้เห็น

## Root

root-unreachable = เชื่อมต่อเซิร์ฟเวอร์ Hoard ไม่ได้: { $error }
root-unreachable-desktop = Hoard ไม่ตอบสนอง: { $error }
root-desktop-reopen = หน้าต่างนี้ออกจากระบบแล้ว ปิด Hoard แล้วเปิดใหม่อีกครั้งเพื่อใช้งานต่อ
root-try-again = ลองอีกครั้ง
root-lazy-connect = การเชื่อมต่อแอป
window-minimize = ย่อเล็กสุด
window-maximize = ขยายใหญ่สุด
window-restore = คืนค่าลง
window-close = ปิด

## แอป: คลัง แถบด้านบน และตัวกรอง

app-lazy-settings = การตั้งค่า
app-lazy-archive = ที่เก็บถาวร
app-lazy-shortcuts = ทางลัด
app-lazy-setup = การตั้งค่าเริ่มต้น
app-theme-system = ตามธีมของระบบ
app-theme-light = ธีมสว่าง
app-theme-dark = ธีมมืด
app-color-red = แดง
app-color-orange = ส้ม
app-color-yellow = เหลือง
app-color-green = เขียว
app-color-teal = เขียวอมฟ้า
app-color-blue = น้ำเงิน
app-color-purple = ม่วง
app-color-pink = ชมพู
app-color-brown = น้ำตาล
app-color-sand = ทราย
app-color-gray = เทา
app-color-slate = เทาอมฟ้า
app-color-black = ดำ
app-color-white = ขาว
app-color-swatch = { $color }: แอสเซ็ตที่มีสี{ $lower }อยู่มาก
app-color-any = สีใดก็ได้
app-color-pick = เลือกสีใดก็ได้
app-color = สี
app-music = เพลง
app-tempo = BPM
app-tempo-min = จาก
app-tempo-max = ถึง
app-tempo-min-title = เพลงที่มีจังหวะเท่านี้หรือเร็วกว่า เป็นจำนวนบีตต่อนาที (นับครึ่งหรือสองเท่าด้วย)
app-tempo-max-title = เพลงที่มีจังหวะเท่านี้หรือช้ากว่า เป็นจำนวนบีตต่อนาที
app-key = คีย์
app-key-any = คีย์ใดก็ได้
app-key-major = เมเจอร์
app-key-minor = ไมเนอร์
app-key-mix = และคีย์ที่มิกซ์เข้ากันได้
app-key-mix-title = คีย์เมเจอร์หรือไมเนอร์สัมพันธ์ และคีย์ที่ห่างกันขั้นคู่ห้าทั้งสองด้าน ซึ่งดีเจใช้มิกซ์ด้วยกัน
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = พิมพ์เสียงตามที่คุณจะสะกด (เช่น whooom) เสียงที่มีรูปแบบคล้ายกันจะขึ้นก่อน
app-imitation-record = ทำเสียงใส่ไมโครโฟน เสียงที่มีรูปแบบคล้ายกันจะขึ้นก่อน
app-imitation-stop = หยุดบันทึก
app-imitation-heard = เสียงของคุณ
app-imitation-mic-failed = บันทึกจากไมโครโฟนไม่ได้: { $error }
app-retry = ลองใหม่
app-undo = เลิกทำ
app-search-failed = ค้นหาไม่สำเร็จ: { $error }
app-unreachable = เชื่อมต่อเซิร์ฟเวอร์ไม่ได้: { $error }
app-unreachable-desktop = Hoard ไม่ตอบสนอง: { $error }
app-forget-project-title = ลืม { $name } หรือไม่?
app-forget-project-body = Hoard จะหยุดแสดงโปรเจกต์นี้ในที่ที่ใช้แอสเซ็ต { $count } รายการของโปรเจกต์ จนกว่า hoard sync ครั้งถัดไปจะแจ้งอีกครั้ง ไม่มีสิ่งใดในโปรเจกต์เปลี่ยนแปลง
app-forget-project-confirm = ลืมโปรเจกต์
app-forget-project-failed = ลืม { $name } ไม่ได้: { $error }
app-forget-project-label = ลืมโปรเจกต์ { $name }
app-forget-project = ลืมโปรเจกต์นี้
app-save-name-like = คล้าย { $name }
app-save-name-favorites = รายการโปรด
app-search-saved = บันทึก “{ $name }” แล้ว: อยู่ในตัวกรอง ใต้หัวข้อ ของคุณ
app-search-updated = อัปเดต “{ $name }” แล้ว
app-rank-alike = คล้ายที่สุด
app-rank-color = มีสีนี้มากที่สุด
app-rank-match = ตรงที่สุด
app-rank-imitation = เสียงใกล้เคียงที่สุด
app-sort = เรียง
app-sort-name = ชื่อ
app-sort-created = วันที่สร้าง
app-sort-updated = วันที่อัปเดต
app-sort-size = ขนาด
app-sort-rating = คะแนน
app-sort-newest-first = ใหม่สุดก่อน
app-sort-oldest-first = เก่าสุดก่อน
app-sort-a-to-z = A ถึง Z
app-sort-z-to-a = Z ถึง A
app-sort-largest-first = ใหญ่สุดก่อน
app-sort-smallest-first = เล็กสุดก่อน
app-sort-best-rated-first = คะแนนสูงสุดก่อน
app-sort-lowest-rated-first = คะแนนต่ำสุดก่อน
app-collection-added = เพิ่ม { $count } แอสเซ็ตลงใน { $name } แล้ว
app-collection-add-failed = เพิ่มลงในคอลเลกชันไม่ได้: { $error }
app-collection-removed = นำ { $count } รายการออกจาก { $name } แล้ว
app-collection-remove-failed = นำออกจากคอลเลกชันไม่ได้: { $error }
app-ns-subject = หัวเรื่อง
app-ns-style = สไตล์
app-ns-theme = ธีม
app-ns-material = แมทีเรียล
app-ns-map = แมป PBR
app-ns-color = สี
app-ns-mood = อารมณ์
app-ns-genre = แนว
app-ns-license = สัญญาอนุญาต
app-ns-res = ความละเอียด
app-ns-duration = ความยาว
app-ns-poly = จำนวนโพลีกอน
app-ns-format = รูปแบบ
app-ns-weight = น้ำหนัก
app-ns-tech = ด้านเทคนิค
app-ns-blender = เวอร์ชัน Blender
app-ns-depth = ความลึกบิต
app-ns-ucs = หมวดเสียง (UCS)
app-ns-bpm = จังหวะ
app-ns-root = โน้ตหลัก
app-ns-level = ความดัง
app-ns-ue = คลาส Unreal
app-ns-engine = เอนจิน
app-ns-extracted = ซอร์สที่กู้คืนได้
app-ns-script = ระบบการเขียน
app-bulk-failed = เปลี่ยนแอสเซ็ต { $count } รายการนั้นไม่ได้: { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied = คัดลอก { $count } แท็กแล้ว
app-tags-copy-none = { $name } ไม่มีแท็กให้คัดลอก
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = ยังไม่ได้คัดลอกแท็ก กด C บนแอสเซ็ตเพื่อคัดลอกแท็กของมัน
# Pasting tags that every one of the assets already has.
app-tags-paste-had = แอสเซ็ต { $count } รายการมีแท็กเหล่านั้นอยู่แล้ว
# After pasting: $count tags were added to $assets assets.
app-tags-pasted = เพิ่ม { $count } แท็กให้ { $assets } แอสเซ็ตแล้ว
app-bulk-untagged = นำแท็ก { $tag } ออกจาก { $count } แอสเซ็ตแล้ว
app-bulk-tagged = ติดแท็ก { $tag } ให้ { $count } แอสเซ็ตแล้ว
app-bulk-review = ทำเครื่องหมายว่าตรวจแล้ว
app-bulk-review-title = ทำเครื่องหมายแอสเซ็ตที่เลือกว่าดูแล้ว: หลังจากนี้กฎและ AI จะไม่แตะแท็กของแอสเซ็ตเหล่านั้น
app-bulk-unreview = ยกเลิกว่าตรวจแล้ว
app-bulk-unreview-title = เอาเครื่องหมายตรวจแล้วออกจากแอสเซ็ตที่เลือก
app-bulk-reviewed = ทำเครื่องหมาย { $count } แอสเซ็ตว่าตรวจแล้ว
app-bulk-unreviewed = เอาเครื่องหมายตรวจแล้วออกจาก { $count } แอสเซ็ต
app-bulk-accept = เก็บแท็กที่เดา
app-bulk-accept-title = เก็บทุกแท็กที่กฎและ AI ใส่ให้แอสเซ็ตที่เลือกไว้เป็นแท็กของคุณเอง
app-bulk-accepted = เก็บแท็กที่เดาของ { $count } แอสเซ็ตไว้แล้ว
app-credits-failed = สร้างเครดิตไม่ได้: { $error }
app-archived-assets = เก็บถาวร { $count } แอสเซ็ตแล้ว
app-archived-one = เก็บถาวร { $name } แล้ว
app-open-failed = เปิดแอสเซ็ตนั้นไม่ได้: { $error }
app-unnamed-pack = แพ็ก
app-unnamed-collection = คอลเลกชัน
app-unnamed-asset = แอสเซ็ต
app-home-version = Hoard { $version } — กลับไปที่คลัง
app-home = กลับไปที่คลัง
app-search-label = ค้นหาแอสเซ็ต
app-search-placeholder = ค้นหาชื่อ พาธ แท็ก
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = หิน or ก้อนหิน: อย่างใดอย่างหนึ่ง · -มอส: ไม่รวม · "มอสแห้ง": คำเหล่านี้ด้วยกัน · pack:ป่า: ในชื่อแพ็ก
app-alt-leave-out = Alt-คลิกเพื่อไม่รวม
app-views = มุมมอง
app-library = คลัง
app-packs = แพ็ก
app-packs-working = กำลังทำงาน { $count }
app-update-title = Hoard { $version } ออกแล้ว (เซิร์ฟเวอร์นี้ใช้ { $current }) ดึงอิมเมจหรือไบนารีใหม่แล้วรีสตาร์ต ฐานข้อมูลจะอัปเกรดตัวเอง
app-update = อัปเดต
app-theme = ธีม
app-sign-in = ลงชื่อเข้าใช้
app-archive-title = ที่เก็บถาวร: { $contents }
app-archive = ที่เก็บถาวร
app-settings = การตั้งค่า
app-palette-title = แผงคำสั่ง ({ $keys })
app-import-folder = นำเข้าโฟลเดอร์
app-add-files = เพิ่มไฟล์
app-add-files-short = เพิ่ม
app-split-label = วิธีนำเข้าโฟลเดอร์นี้
app-split-before = { "" }
app-split-after = { " " }มี { $count } โฟลเดอร์หรือไฟล์บีบอัด
app-split-each = นำเข้าแต่ละอันเป็นแพ็กของตัวเอง ({ $count })
app-split-one = นำเข้าเป็นแพ็กเดียว
app-cancel = ยกเลิก
app-filters = ตัวกรอง
app-done = เสร็จ
app-yours = ของคุณ
app-favorites = รายการโปรด
app-needs-review = ต้องตรวจ
app-needs-review-title = เฉพาะแอสเซ็ตที่ยังไม่มีใครตรวจ (ใช้ร่วมกับตัวกรองอื่นได้)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = ให้คะแนน
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
       *[other] { $count } ดาวขึ้นไป
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = คลิกอีกครั้งเพื่อแสดงทุกคะแนน
app-forget-search-label = ลืมการค้นหาที่บันทึกไว้ { $name }
app-forget-search = ลืมการค้นหานี้
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = เปิดคลังด้วยการค้นหานี้
app-default-search-on-title = คลังจะเปิดด้วยการค้นหานี้ คลิกเพื่อยกเลิก
app-default-search-label = เปิดคลังด้วย { $name }
app-rename-search = เปลี่ยนชื่อ
app-rename-search-label = ชื่อใหม่ของ { $name }
app-projects-title = โปรเจกต์ที่ใช้ hoard sync จากที่นี่ พร้อมสิ่งที่การซิงก์ครั้งล่าสุดมี
app-projects = โปรเจกต์
app-project-synced = ซิงก์เมื่อ { $when }
app-project-synced-by = ซิงก์เมื่อ { $when } โดย { $who }
app-collections = คอลเลกชัน
app-no-filters = ไม่มีตัวกรองสำหรับการค้นหานี้
app-kind = ประเภท
app-facets-more = อีก { $count } รายการ
app-facets-fewer = น้อยลง
app-searching = กำลังค้นหา…
app-searching-live = กำลังค้นหา
app-asset-count = { $count } แอสเซ็ต
app-clear-all = ล้างทั้งหมด
app-save-name-label = ชื่อของการค้นหานี้
app-save-name-placeholder = ตั้งชื่อ
app-save = บันทึก
app-save-search-title = เก็บการค้นหานี้ไว้ในตัวกรอง ใต้หัวข้อ ของคุณ
app-save-search = บันทึกการค้นหา
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = อัปเดต “{ $name }”
app-update-search-title = เก็บสิ่งที่เลือกอยู่ตอนนี้ไว้ในการค้นหาที่บันทึกนี้
app-save-search-new = บันทึกเป็นรายการใหม่
app-select-title = เลือกหลายแอสเซ็ตเพื่อติดแท็ก จัดหมวดหมู่ เพิ่มลงคอลเลกชัน ดาวน์โหลด หรือเก็บถาวรพร้อมกัน (X หรือ Ctrl-คลิก)
app-select = เลือก
app-tile-size = ขนาดไทล์
app-supporting-title = บัฟเฟอร์ glTF, ไลบรารีแมทีเรียล, แมปของแมทีเรียล, แมทีเรียลที่มีโมเดลเดียวใช้ และไฟล์ร้านค้าที่อธิบายโมเดล ไฟล์เหล่านี้จะมาด้วยเมื่อคุณดาวน์โหลดสิ่งที่ใช้ไฟล์เหล่านี้ หน้าของสไปรต์ชีต และสไปรต์ของชีตที่แพ็กมีเป็นภาพแยกด้วย จะแสดงร่วมกับชีต แอสเซ็ตที่แพ็กมีหลายรูปแบบจะแสดงครั้งเดียว ในรูปแบบที่ Hoard อ่านได้ดีที่สุด
app-supporting-hidden = ซ่อนไฟล์ประกอบ { $count } ไฟล์อยู่
app-supporting-show = แสดง
app-supporting-shown = รวมไฟล์ประกอบด้วย
app-supporting-hide = ซ่อน
app-folders = โฟลเดอร์
app-folders-title = ดูแพ็กตามที่อยู่บนดิสก์ ทีละโฟลเดอร์
app-folders-label = โฟลเดอร์ของแพ็ก
app-welcome-title = วางแพ็กแอสเซ็ตที่ใดก็ได้บนหน้านี้
app-welcome-before = ใช้ได้ทั้งไฟล์ zip, tarball, โฟลเดอร์ หรือไฟล์เดี่ยว ๆ Hoard จะแตกไฟล์ จัดเก็บทุกไฟล์เพียงครั้งเดียวตามแฮช อ่านขนาด ความยาว และจำนวนโพลีกอน แล้วติดแท็กทุกอย่างเพื่อให้คุณค้นหาสิ่งต่าง ๆ ได้ เช่น{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }หรือ{ " " }
app-welcome-after = { "" }
app-welcome-import-folder = นำเข้าโฟลเดอร์
app-welcome-choose-files = เลือกไฟล์
app-none-similar = ยังไม่มีสิ่งอื่นที่ดูคล้าย { $name } มากนัก
app-none = ไม่มีรายการที่ตรงกัน
app-none-query = ไม่มีรายการที่ตรงกับ “{ $query }”
app-none-imitation = ยังไม่มีเสียงที่วัดแล้วคล้าย “{ $word }”
app-none-heard = ยังไม่มีเสียงที่วัดแล้วคล้ายเสียงของคุณ
app-none-filters = ไม่มีรายการที่ตรงกับตัวกรองเหล่านี้
app-none-query-filters = ไม่มีรายการที่ตรงกับ “{ $query }” ด้วยตัวกรองเหล่านี้
app-clear-search-filters = ล้างการค้นหาและตัวกรอง
app-loading-more = กำลังโหลดเพิ่ม
app-keys-hint = ปุ่มลูกศรเพื่อเลื่อน Enter เพื่อเปิด Space เพื่อเล่นเสียง X หรือ Ctrl-คลิกเพื่อเลือกหลายรายการ (กดค้างบนจอสัมผัส)
app-resize-label = ปรับขนาดแผงรายละเอียด
app-resize-title = ลากเพื่อปรับขนาด · ดับเบิลคลิกเพื่อใช้ความกว้างที่พอดี
app-selected-label = แอสเซ็ตที่เลือก
app-selected = เลือก { $count } รายการ
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = แก้ไข…
app-edit-selected-title = เปลี่ยนแท็กและช่องข้อมูลของทั้งหมด (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
       *[other] เลือกผลลัพธ์ { $count } รายการ
    }
app-pick-assets = เลือกแอสเซ็ต
app-download-zip = ดาวน์โหลด zip
app-credits-title = CREDITS.md ของแพ็กที่แอสเซ็ตเหล่านี้มาจาก: ใครเป็นผู้สร้าง สัญญาอนุญาต และข้อความเครดิตที่ผู้สร้างขอ
app-credits = เครดิต
app-prefer-sources-title = SoundWave และเท็กซ์เจอร์ของ Unreal ที่ Hoard กู้ไฟล์ต้นฉบับคืนมาได้
app-prefer-sources = WAV/PNG ที่กู้คืนได้แทน .uasset ({ $count })
bag-clear = ล้างที่เลือก
bag-glb-heading = โมเดลเป็น GLB
bag-glb-text = ทุกโมเดลเป็นไฟล์ .glb ที่ตั้งตรงโดยให้แกน Y ชี้ขึ้นและหันไปด้านหน้า พร้อมเท็กซ์เจอร์เป็นไฟล์แยกในโฟลเดอร์ textures ที่โมเดลใช้ร่วมกัน ส่วนไฟล์อื่นใส่ไปตามเดิม
bag-glb-download = ดาวน์โหลด zip ของ GLB
bag-plain-heading = ไฟล์ตามเดิม
bag-plain-text = แต่ละไฟล์ตามที่นำเข้ามา พร้อมไฟล์ที่ต้องใช้ ในโฟลเดอร์แยกตามแพ็ก
bag-remove = นำ { $name } ออกจากที่เลือก
bag-more = และอีก { $count } รายการ
app-bulk-tag-label = แท็กที่จะเพิ่มหรือนำออกจากแอสเซ็ตที่เลือก
app-bulk-tag-placeholder = แท็ก
app-bulk-tag-add-title = เพิ่มแท็กนี้ให้ทุกแอสเซ็ตที่เลือก
app-bulk-tag-add = เพิ่ม
app-bulk-tag-remove-title = นำแท็กนี้ออกจากทุกแอสเซ็ตที่เลือก
app-bulk-tag-remove = นำออก
app-remove-from-collection = นำออกจาก { $name }
app-unfavorite = นำออกจากรายการโปรด
app-favorite = เพิ่มในรายการโปรด
app-select-all = เลือกทั้งหมด { $count } รายการ
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
       *[other] เลือกผลลัพธ์ทั้ง { $count } รายการ
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
       *[other] แก้ไขแอสเซ็ต { $count } รายการ
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
       *[other] แก้ไขผลลัพธ์ทั้ง { $count } รายการ
    }
bulk-edit-close = ปิด
bulk-edit-tags = แท็ก
bulk-edit-tag-add = เพิ่มแท็กให้ทั้งหมด
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } จาก { $count }
bulk-edit-untick = ยกเลิกเลือกทั้งหมด
bulk-edit-untick-title = เอาแท็กออกทั้งหมด แล้วเลือกเฉพาะแท็กที่จะเก็บไว้
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = แท็กที่ใช้บ่อยที่สุดในกลุ่มนี้
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = ค่าต่างกัน คงไว้ตามเดิม
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = คงไว้ตามเดิม
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = ล้างในทั้งหมดแล้ว
bulk-edit-reset = คืนค่าเดิม
bulk-edit-all-note = การเปลี่ยนนี้มีผลกับผลลัพธ์ทั้งหมด รวมถึงที่ยังไม่โหลด และย้อนกลับไม่ได้ในขั้นตอนเดียว
bulk-edit-save =
    { $count ->
       *[other] บันทึก { $count } การเปลี่ยนแปลง
    }
bulk-edit-nothing = ไม่มีอะไรเปลี่ยนแปลง
bulk-edit-saved =
    { $count ->
       *[other] อัปเดตแอสเซ็ต { $count } รายการแล้ว
    }
app-archive-selected-title = ไม่แสดงในการค้นหาจนกว่าจะกู้คืนหรือลบถาวรจากที่เก็บถาวร
app-drop-items = วาง { $count } รายการเพื่อเพิ่ม
app-drop = วางเพื่อเพิ่มลงใน Hoard
app-drop-hint = ไฟล์หรือโฟลเดอร์ระดับบนสุดแต่ละรายการจะกลายเป็นแพ็กของตัวเอง ไฟล์ขนาดหลายกิกะไบต์ก็ได้ ไฟล์ใหญ่จะอัปโหลดต่อได้หากการเชื่อมต่อหลุด

## แผงคำสั่ง (CommandPalette.vue พร้อมคำสั่งจาก App.vue)

palette-group-go-to = ไปที่
palette-group-actions = การทำงาน
palette-group-theme = ธีม
palette-group-news = มีอะไรใหม่
palette-news-show = ดูว่ามีอะไรใหม่
palette-news-mark-all = ทำเครื่องหมายสิ่งใหม่ทั้งหมดว่าเห็นแล้ว
palette-news-keywords = ใหม่ ข่าว การเปลี่ยนแปลง ฟีเจอร์ รุ่น อัปเดต เห็นแล้ว new news
palette-group-kinds = ประเภท
palette-group-packs = แพ็ก
palette-group-collections = คอลเลกชัน
palette-group-projects = โปรเจกต์
palette-group-saved = การค้นหาที่บันทึกไว้
palette-group-assets = แอสเซ็ต
palette-hint-current = ปัจจุบัน
palette-library = คลัง
palette-library-keywords = แอสเซ็ต ตาราง หน้าแรก assets grid home
palette-favorites = รายการโปรด
palette-favorites-keywords = ดาว ชอบ โปรด starred stars liked
palette-packs = แพ็ก
palette-archive = ที่เก็บถาวร
palette-archive-keywords = ถังขยะ ลบ กู้คืน trash recycle bin deleted restore
palette-stats = สถิติคลัง
palette-stats-keywords = สถิติ ตัวเลข ขนาด พื้นที่ดิสก์ จำนวน statistics numbers size disk space count
palette-copies = สำเนาที่เกือบเหมือนกัน
palette-copies-keywords = ซ้ำ เหมือนกัน คล้าย ย่อขนาด แปลง พื้นที่เปลือง duplicates same similar resized converted wasted space dedupe
palette-search = ค้นหา
palette-add-files = เพิ่มไฟล์…
palette-add-files-keywords = อัปโหลด นำเข้า upload import
palette-import-folder = นำเข้าโฟลเดอร์…
palette-import-folder-keywords = อัปโหลด แพ็ก upload pack
palette-select-all = เลือกทั้งหมด { $count } รายการที่แสดง
palette-select-all-keywords = เลือก ติ๊ก check
palette-select-several = เลือกหลายรายการ…
palette-select-several-keywords = หลายรายการ แท็ก เก็บถาวร bulk multiple tag archive
palette-archive-selected = เก็บถาวร { $count } รายการที่เลือก
palette-archive-selected-keywords = ลบ นำออก ถังขยะ delete remove trash
palette-edit-selected = แก้ไข { $count } รายการที่เลือก
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = เปลี่ยนจำนวนมาก แท็ก หมวดหมู่ ใบอนุญาต คะแนน
palette-paste-tags = วางแท็กที่คัดลอกไว้ลงใน { $count } รายการที่เลือก
palette-clear-selection = ล้างการเลือก
palette-clear-filters = ล้างตัวกรอง
palette-clear-filters-keywords = รีเซ็ต reset
palette-hide-supporting = ซ่อนไฟล์ประกอบ
palette-show-supporting = แสดงไฟล์ประกอบ
palette-supporting-keywords = นอร์มอลแมป ความหยาบ มาสก์ normal maps roughness masks
palette-account = บัญชีและโทเค็นการเข้าถึง
palette-account-keywords = รหัสผ่าน ล็อกอิน ผู้ใช้ โทเค็น password login user token api
palette-sign-out = ลงชื่อออก ({ $name })
palette-sign-out-keywords = ออกจากระบบ ล็อกเอาต์ log out logout
palette-sign-in = ลงชื่อเข้าใช้
palette-sign-in-keywords = เข้าสู่ระบบ ล็อกอิน บัญชี รหัสผ่าน log in login account password
palette-settings = การตั้งค่า
palette-settings-keywords = ผู้ดูแลระบบ เครื่องมือ ค่ากำหนด แท็ก blender ffmpeg llm ai tagging unreal admin tools preferences
palette-shortcuts = แป้นพิมพ์ลัด
palette-shortcuts-keywords = ช่วยเหลือ ปุ่ม help keys
palette-theme-keywords = ธีม รูปลักษณ์ โหมด theme appearance mode
palette-theme-variant-keywords = รูปลักษณ์ สี appearance color
palette-light-theme = ธีมสว่าง: { $name }
palette-dark-theme = ธีมมืด: { $name }
palette-kind-all = ทุกประเภท (เลิกแสดงเฉพาะ{ $kind })
palette-kind-only = เฉพาะ{ $kind }
palette-title = แผงคำสั่ง
palette-placeholder = ค้นหาแอสเซ็ต ไปที่แพ็ก เรียกใช้คำสั่ง…
palette-results = ผลลัพธ์
palette-searching = กำลังค้นหา…
palette-no-match = ไม่มีรายการที่ตรงกับ “{ $query }”
palette-foot-move = เลื่อน
palette-foot-run = เรียกใช้
palette-foot-shortcuts = ทางลัดทั้งหมด

## แป้นพิมพ์ลัด (ShortcutsDialog.vue)

shortcuts-title = แป้นพิมพ์ลัด
shortcuts-close = ปิด
shortcuts-anywhere = ทุกที่
shortcuts-palette = แผงคำสั่ง: ค้นหา ข้ามไป เรียกใช้
shortcuts-search = ค้นหา
shortcuts-this-list = รายการนี้
shortcuts-escape = ปิด ล้างการเลือก หรือปิดรายละเอียด
shortcuts-library = คลัง
shortcuts-move = เลื่อนระหว่างไทล์ (หรือ J และ K)
shortcuts-open = เปิดรายละเอียด
shortcuts-play-sound = เล่นหรือหยุดเสียง
shortcuts-select = เลือกหรือยกเลิกการเลือก
shortcuts-favorite = เพิ่มหรือนำออกจากรายการโปรด
shortcuts-reviewed = ทำเครื่องหมายว่าตรวจแล้ว หรือยกเลิก
shortcuts-tag = เพิ่มแท็ก
shortcuts-edit = แก้ไขรายการที่เลือก
shortcuts-copy-tags = คัดลอกแท็กของมัน
shortcuts-paste-tags = วางแท็กที่คัดลอกไว้ลงในรายการที่เลือก
shortcuts-key-click = คลิก
shortcuts-select-range = เลือกเป็นช่วง
shortcuts-select-all = เลือกทุกรายการที่แสดง
shortcuts-sound-player = เครื่องเล่นเสียง
shortcuts-play-pause = เล่นหรือหยุดชั่วคราว
shortcuts-skip = ข้ามถอยหลังหรือไปข้างหน้า
shortcuts-start-end = ไปต้นหรือท้าย
shortcuts-image-inspector = ตัวตรวจดูภาพ
shortcuts-zoom = ซูมเข้าหรือออก
shortcuts-fit = พอดีกับหน้าต่าง
shortcuts-actual-pixels = พิกเซลจริง
shortcuts-tile = เรียงต่อ 3×3 เพื่อตรวจรอยต่อ
shortcuts-background = พื้นหลังถัดไป
shortcuts-channel = แสดงช่องสีเดียว

## KindChips.vue

kinds-group = ประเภทแอสเซ็ต

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = เพิ่มโดยผู้ใช้
tag-source-rule = จากตัวติดแท็กออฟไลน์
tag-source-meta = วัดจากไฟล์
tag-source-ai = แนะนำโดย AI
tag-remove = นำแท็ก { $tag } ออก
tag-accept = เก็บแท็ก { $tag } ไว้
tag-accept-title = เก็บแท็กที่เดามานี้ไว้เป็นแท็กของคุณเอง
tag-reject-title = นำออก; ตัวติดแท็กจะไม่ใส่กลับมาอีก
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = แท็กใหม่
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = คุณหมายถึง
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = เดิม { $alias }

## ActivityIndicator.vue

activity-label-idle = กิจกรรมเบื้องหลัง: ว่าง
activity-label-running = กิจกรรมเบื้องหลัง: กำลังทำงาน { $running }
activity-label-queued = กิจกรรมเบื้องหลัง: กำลังทำงาน { $running } รอคิว { $queued }
activity-title = กิจกรรมเบื้องหลัง
activity-unreachable = เชื่อมต่อเซิร์ฟเวอร์ไม่ได้
activity-unreachable-desktop = Hoard ไม่ตอบสนอง
activity-empty = ไม่มีอะไรทำงานอยู่ การนำเข้า การติดแท็ก การแปลง และการดาวน์โหลด zip จะแสดงที่นี่ขณะทำงาน
activity-queued = รอคิว
activity-stop = หยุด “{ $what }”
activity-stop-tagging = หยุดการติดแท็กด้วย AI ทั้งหมด ({ $count })
activity-seconds = { $s } วินาที
activity-minutes = { $min } นาที
activity-hours = { $h } ชม. { $min } นาที
activity-percent = { $pct }%

## AssetTile.vue

tile-triangles = { $count } สามเหลี่ยม
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = เล่น { $name }
tile-pause = หยุด { $name } ชั่วคราว
tile-favorite = เพิ่ม { $name } ในรายการโปรด
tile-in-favorites = อยู่ในรายการโปรดของคุณ
tile-add-favorite = เพิ่มในรายการโปรด (F)
tile-more-like = เพิ่มเติมที่คล้าย { $name }
tile-more-like-this = เพิ่มเติมที่คล้ายกัน
tile-select = เลือก { $name }

## CollectionPicker.vue

collections-add = เพิ่มลงในคอลเลกชัน
collections-menu = คอลเลกชัน
collections-find-label = ค้นหาหรือสร้างคอลเลกชัน
collections-find-placeholder = ค้นหาหรือตั้งชื่อคอลเลกชันใหม่
collections-create = สร้าง
collections-empty = ยังไม่มีคอลเลกชัน พิมพ์ชื่อเพื่อเริ่มสร้าง

## FolderPicker.vue

folders-list = โฟลเดอร์
folders-close = ปิด
folders-browse = เรียกดู…
folders-up-title = ขึ้นไปหนึ่งโฟลเดอร์
folders-up = ขึ้น
folders-this-computer = คอมพิวเตอร์นี้
folders-none = ไม่มีโฟลเดอร์ในนี้

## LocationForm.vue

locations-mode-question = Hoard ควรทำอะไรกับโฟลเดอร์นี้?
locations-mode-managed = ยึดไว้: Hoard เก็บแพ็กของฉันไว้ที่นี่
locations-mode-managed-hint = แพ็กที่คุณอัปโหลดหรือนำเข้าจะถูกคัดลอกมาไว้ที่นี่ และ Hoard จะจัดให้เป็นระเบียบ ตอนนี้โฟลเดอร์เป็นของ Hoard แล้ว จึงอย่าแตะต้องสิ่งที่อยู่ข้างใน
locations-mode-watched = เฝ้าดู: แพ็กของฉันอยู่ที่นี่แล้ว
locations-mode-watched-hint = แต่ละโฟลเดอร์ข้างในจะกลายเป็นแพ็ก Hoard อ่านไฟล์จากที่เดิมและไม่เปลี่ยน ย้าย หรือลบไฟล์เลย และจะรับโฟลเดอร์ใหม่เมื่อปรากฏขึ้น
locations-folder-desktop = โฟลเดอร์
locations-folder-placeholder = โฟลเดอร์ที่แพ็กของคุณอยู่
locations-folder-hint-managed = Hoard จะสร้างโฟลเดอร์ให้หากยังไม่มี ใน Docker นี่คือโฟลเดอร์ที่คุณเมานต์เข้าไปในคอนเทนเนอร์
locations-folder-hint-managed-desktop = Hoard จะสร้างโฟลเดอร์ให้หากยังไม่มี
locations-folder-hint-watched = ใน Docker ให้เมานต์โฟลเดอร์เข้าไปในคอนเทนเนอร์ก่อน (แบบอ่านอย่างเดียวก็ได้) แล้วเลือกที่นี่
locations-folder-hint-watched-desktop = Hoard อ่านโฟลเดอร์นี้อย่างเดียว ไม่มีสิ่งใดในนั้นถูกเปลี่ยนแปลง
locations-name = ชื่อ
locations-name-placeholder-managed = คลัง
locations-name-placeholder-watched = แพ็กของฉัน
locations-files = ไฟล์ที่จะมาอยู่ที่นี่
locations-catch-all = รวมถึงทุกอย่างที่ตำแหน่งอื่นไม่รับ
locations-kinds-hint = ไฟล์ของแพ็กใหม่จะไปยังตำแหน่งที่ติ๊กประเภทของไฟล์นั้นไว้ ส่วนที่เหลือจะไปยังตำแหน่งที่รับทุกอย่าง
locations-files-all = ทั้งหมดไปก่อน เพิ่มตำแหน่งอื่นภายหลังได้จากการตั้งค่า เพื่อส่งบางประเภท (เช่น เพลง) ไปยังดิสก์อื่น
locations-first-watched-note = การอัปโหลดต้องมีคลังที่ Hoard ดูแล คุณเพิ่มได้ภายหลังจาก การตั้งค่า → ตำแหน่งคลัง
locations-checking = กำลังตรวจสอบโฟลเดอร์…
locations-add = เพิ่มตำแหน่ง
locations-cancel = ยกเลิก

## LocationsPanel.vue

locations-failed = { $what }: { $error }
locations-load-failed = โหลดตำแหน่งคลังไม่ได้
locations-takes-and-rest = { $kinds } และทุกอย่างที่เหลือ
locations-takes-rest = ทุกอย่างที่ตำแหน่งอื่นไม่รับ
locations-takes-everything = ทุกอย่าง
locations-takes-nothing = ยังไม่มี
locations-save-failed = บันทึกตำแหน่งไม่ได้
locations-unwatch-title = หยุดเฝ้าดู { $name } หรือไม่?
locations-unwatch-body = แพ็ก { $count } แพ็กของโฟลเดอร์นี้จะออกจาก Hoard พร้อมแท็กและโน้ต ไฟล์ในโฟลเดอร์จะไม่ถูกแตะต้อง
locations-unwatch-body-empty = ไฟล์ในโฟลเดอร์จะไม่ถูกแตะต้อง
locations-unwatch = หยุดเฝ้าดู
locations-remove-title = นำ { $name } ออกหรือไม่?
locations-remove-body = ไฟล์ทั้งหมดในตำแหน่งนี้จะย้ายไปยังตำแหน่งอื่นตามประเภทก่อน แล้วตำแหน่งนี้จะถูกนำออกเมื่อว่างเปล่า ไม่มีสิ่งใดสูญหาย
locations-remove-body-only = หากตำแหน่งนี้มีไฟล์อยู่ ให้เพิ่มตำแหน่งอื่นเพื่อย้ายไฟล์ไปก่อน
locations-remove = นำออก
locations-removed = นำ { $name } ออกแล้ว
locations-emptying = กำลังย้ายไฟล์ออกจาก { $name } จะนำออกเมื่อเสร็จ
locations-offline-title = เข้าถึง { $name } ไม่ได้
locations-offline-body = ย้ายไฟล์ออกไม่ได้ขณะที่ตำแหน่งนี้ออฟไลน์ นำออกเลยหรือไม่? ไฟล์ที่ยังอยู่ในนั้นจะแสดงว่าหายไป
locations-remove-anyway = นำออกเลย
locations-remove-failed = นำ { $name } ออกไม่ได้
locations-rebalancing = กำลังย้ายไฟล์ที่จัดเก็บไว้ไปยังตำแหน่งที่รับประเภทของไฟล์นั้น
locations-rebalance-failed = ย้ายไฟล์ไม่ได้
locations-scanned-never = ยังไม่ได้สแกน
locations-scanned-now = สแกนเมื่อสักครู่
locations-scanned-minutes = สแกนเมื่อ { $mins } นาทีที่แล้ว
locations-scanned-hours = สแกนเมื่อ { $hours } ชม. ที่แล้ว
locations-scanned-date = สแกนเมื่อ { $date }
locations-scan-failed = สแกน { $name } ไม่ได้
locations-watching = กำลังเฝ้าดู { $name }
locations-added = เพิ่ม { $name } แล้ว
locations-loading = กำลังโหลด…
locations-managed-title = คลังที่ Hoard ดูแล
locations-watched-title = โฟลเดอร์ที่เฝ้าดู
locations-managed-hint = สิ่งที่อัปโหลดและนำเข้าจะถูกคัดลอกมาไว้ที่นี่ แต่ละตำแหน่งรับประเภทที่ติ๊กไว้ ตำแหน่งที่รับทุกอย่างจะรับส่วนที่เหลือ
locations-watched-hint = โฟลเดอร์ของแพ็กที่จัดทำดัชนีจากที่เดิม Hoard ไม่เปลี่ยนไฟล์ในโฟลเดอร์เหล่านี้เลย และจะตรวจหาโฟลเดอร์ใหม่และที่ถูกนำออกทุก 15 นาที
locations-managed-empty = ยังไม่มี จึงไม่มีที่ให้เก็บสิ่งที่อัปโหลด เพิ่มตำแหน่งด้านล่าง
locations-reachable = เข้าถึงได้
locations-unreachable-now = เข้าถึงไม่ได้ในตอนนี้
locations-catch-all-tag = รับทุกอย่าง
locations-packs = { $count } แพ็ก
locations-unreachable = เข้าถึงไม่ได้
locations-scanning = กำลังสแกน…
locations-scan = สแกนเดี๋ยวนี้
locations-edit = แก้ไข
locations-catch-all-locked = หากต้องการเปลี่ยน ให้ตั้งตำแหน่งอื่นเป็นที่รับทุกอย่าง
locations-edit-hint-before = มีผลกับไฟล์ที่เพิ่มนับจากนี้ หากต้องการย้ายสิ่งที่จัดเก็บไว้แล้ว ให้ใช้
locations-edit-hint-after = ด้านล่าง
locations-save = บันทึก
locations-rebalance = ย้ายไฟล์ที่จัดเก็บไว้ให้ตรงกัน
locations-rebalance-hint = หลังจากเปลี่ยนสิ่งที่ตำแหน่งรับ: ไฟล์ที่จัดเก็บไว้แล้วจะย้ายไปยังตำแหน่งที่ประเภทของไฟล์นั้นต้องไปในตอนนี้
locations-add-title = เพิ่มตำแหน่ง
locations-add-hint = ดิสก์อีกตัวสำหรับไฟล์บางประเภท หรือโฟลเดอร์ของแพ็กที่คุณมีอยู่แล้ว
locations-kind-question = ตำแหน่งแบบไหน
locations-kind-folder = โฟลเดอร์
locations-kind-folder-hint = บนดิสก์หรือโฟลเดอร์แชร์ของ NAS ที่เมานต์เข้าไปในคอนเทนเนอร์ หรือโฟลเดอร์ที่ Dropbox, Google Drive, OneDrive หรือ iCloud Drive ซิงค์ไว้ Hoard เก็บไฟล์ไว้ที่นั่นได้ หรือคอยดูแพ็กที่คุณมีอยู่แล้วก็ได้
locations-kind-folder-hint-desktop = บนคอมพิวเตอร์เครื่องนี้ ดิสก์อื่น หรือโฟลเดอร์แชร์ของ NAS หรือโฟลเดอร์ที่ Dropbox, Google Drive, OneDrive หรือ iCloud Drive ซิงค์ไว้ Hoard เก็บไฟล์ไว้ที่นั่นได้ หรือคอยดูแพ็กที่คุณมีอยู่แล้วก็ได้
locations-kind-s3-hint = บักเก็ตใน Amazon S3 หรือบริการที่คล้ายกัน: Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage Hoard จะเก็บไฟล์ไว้ที่นั่น
locations-s3-title = ที่เก็บข้อมูล S3
locations-s3-hint = ตำแหน่ง s3://bucket-name คือบักเก็ตในบริการนี้ ซึ่งเป็น AWS หรือบริการใดก็ได้ที่เข้ากันได้กับ S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage บน NAS)
locations-bucket = บักเก็ต
locations-bucket-hint = ชื่อบักเก็ตที่มีอยู่แล้ว คีย์ด้านบนต้องแสดงรายการ อ่าน และเขียนบักเก็ตนั้นได้
locations-s3-endpoint = ที่อยู่
locations-s3-endpoint-placeholder = เว้นว่างไว้สำหรับ AWS เช่น https://s3.eu-central-003.backblazeb2.com
locations-s3-region = ภูมิภาค
locations-s3-access-key = รหัสคีย์การเข้าถึง
locations-s3-secret-key = คีย์การเข้าถึงลับ
locations-s3-headers = ส่วนหัวเพิ่มเติม
locations-s3-headers-placeholder = Name=value บรรทัดละหนึ่งรายการ (เช่น สำหรับ Cloudflare Access)
locations-s3-kept = บันทึกแล้ว (พิมพ์เพื่อแทนที่)
locations-s3-secrets = คีย์และส่วนหัวจะเก็บไว้กับการตั้งค่าของเซิร์ฟเวอร์ และจะไม่แสดงอีก
locations-s3-secrets-desktop = คีย์และส่วนหัวจะเก็บไว้ในที่เก็บรหัสผ่านของคอมพิวเตอร์เครื่องนี้ และจะไม่แสดงอีก
locations-s3-saved = บันทึกที่เก็บข้อมูล S3 แล้ว
locations-s3-save-failed = บันทึกที่เก็บข้อมูล S3 ไม่ได้
locations-add-more = เพิ่มตำแหน่ง…

## IssuesPanel.vue

issues-failed = { $what }: { $error }
issues-load-failed = โหลดปัญหาไม่ได้
issues-started = เริ่มใหม่แล้ว { $count } รายการ
issues-retry-failed = ลองใหม่ไม่ได้
issues-dismiss-failed = ปิดไม่ได้
issues-dismiss-all-title = ปิดทั้งหมด { $count } รายการหรือไม่?
issues-dismiss-all-body = “{ $title }” จะถูกล้าง หากมีสิ่งใดผิดพลาดอีกจะแสดงรายการอีกครั้ง
issues-dismiss-all = ปิดทั้งหมด
issues-just-now = เมื่อสักครู่
issues-minutes-ago = { $mins } นาทีที่แล้ว
issues-hours-ago = { $hours } ชม. ที่แล้ว
issues-loading = กำลังโหลด…
issues-none = ไม่มีอะไรผิดพลาด
issues-action-all = { $action }ทั้งหมด
issues-open-asset = เปิด
issues-open-pack = แสดงแพ็ก
issues-open-locations = ตำแหน่งคลัง
issues-where-before = ใน
issues-where-after = { "" }
issues-times = { $count } ครั้ง
issues-starting = กำลังเริ่ม…
issues-dismiss-named = ปิด { $title }
issues-dismiss = ปิด
issues-show-fewer = แสดงน้อยลง
issues-show-all = แสดงทั้งหมด { $count } รายการ
issues-not-listed = มีอีก { $count } รายการที่ไม่ได้แสดง

## archive.ts

archive-anyway = เก็บถาวรเลย
archive-failed = เก็บถาวรไม่ได้: { $error }
archive-undo = เลิกทำ
archive-restore-failed = กู้คืนไม่ได้: { $error }
archive-count-packs-and-assets = { $packs } แพ็กและ { $assets } แอสเซ็ต
archive-count-packs = { $packs } แพ็ก
archive-count-assets = { $assets } แอสเซ็ต
archive-count-nothing = ไม่มีอะไร

## ArchiveView.vue

archive-load-failed = โหลดที่เก็บถาวรไม่ได้: { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = กู้คืน { $what } แล้ว
archive-purged = ลบ { $what } ถาวรแล้ว
archive-purge-failed = ลบไม่ได้: { $error }
archive-purge-title = ลบ { $name } ถาวรหรือไม่?
archive-purge-pack-body = แอสเซ็ต { $count } รายการและไฟล์ขนาด { $size } ของแพ็กนี้จะถูกลบไปด้วย ยกเว้นไฟล์ที่แพ็กอื่นใช้ร่วมกัน การกระทำนี้ย้อนกลับไม่ได้
archive-purge = ลบถาวร
archive-purge-asset-body = ไฟล์ของแอสเซ็ตนี้จะถูกลบออก เว้นแต่แอสเซ็ตอื่นใช้ร่วมกัน การกระทำนี้ย้อนกลับไม่ได้
archive-empty-title = ล้างที่เก็บถาวรหรือไม่?
archive-empty-body = { $what } จะถูกลบถาวร และไฟล์จะถูกลบออกเพื่อคืนพื้นที่ การกระทำนี้ย้อนกลับไม่ได้
archive-empty = ล้างที่เก็บถาวร
archive-title = ที่เก็บถาวร
archive-intro = แพ็กและแอสเซ็ตที่เก็บถาวรจะไม่แสดงในการค้นหา การนับ และรายการแท็ก กู้คืนได้ หรือลบถาวรเพื่อคืนพื้นที่
archive-loading = กำลังโหลด…
archive-is-empty = ที่เก็บถาวรว่างเปล่า
archive-is-empty-hint = เก็บถาวรแพ็กหรือแอสเซ็ตแล้วจะรออยู่ที่นี่จนกว่าคุณจะกู้คืนหรือลบถาวร
archive-packs = แพ็ก
archive-assets = แอสเซ็ต
archive-asset-count = { $count } แอสเซ็ต
archive-archived = เก็บถาวรเมื่อ { $when }
archive-from = จาก { $pack }
archive-restore = กู้คืน
archive-show-more = แสดงเพิ่ม (เหลือ { $count } รายการ)

## mine.ts

projects-favorite-add-failed = เพิ่มในรายการโปรดไม่ได้: { $error }
projects-favorite-remove-failed = นำออกจากรายการโปรดไม่ได้: { $error }
projects-save-search-failed = บันทึกการค้นหาไม่ได้: { $error }
projects-forget-search-failed = นำ { $name } ออกไม่ได้: { $error }
projects-change-search-failed = เปลี่ยน { $name } ไม่ได้: { $error }

## projects.ts

projects-list-and = { $items } และ { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = อีก { $count } รายการ
projects-other = โปรเจกต์อื่นอีก { $count } โปรเจกต์
projects-usage-title-one = { $name } ใช้สิ่งนี้อยู่
projects-usage-title = { $count } โปรเจกต์ใช้สิ่งนี้อยู่
projects-usage-body-one = ใช้อยู่ใน { $list }: hoard sync ครั้งถัดไปของโปรเจกต์จะนำออก
projects-usage-body = ใช้อยู่ใน { $list }: hoard sync ครั้งถัดไปของแต่ละโปรเจกต์จะนำออก

## uploader.ts

upload-network-error = ข้อผิดพลาดของเครือข่าย
upload-gave-up = { $what }: { $error } หลังจากลอง { $attempts } ครั้ง
upload-failed-plain = ล้มเหลว
upload-already-imported = นำเข้าแล้ว
upload-part-failed = อัปโหลดไม่สำเร็จ ({ $status })
upload-batch = ชุดไฟล์ { $count } ไฟล์
upload-batch-failed = ชุดไฟล์ล้มเหลว ({ $status })
upload-finish = การจบการอัปโหลด

## UploadQueue.vue

upload-eta-seconds = เหลือ { $s } วินาที
upload-eta-minutes = เหลือ { $min } นาที
upload-eta-hours = เหลือ { $h } ชม.
upload-queue = การนำเข้า
upload-importing = กำลังนำเข้า { $count } แพ็ก
upload-bytes-of = { $sent } จาก { $total }
upload-rate = { $size }/วินาที
upload-waiting = กำลังรอ
upload-unpacking = อัปโหลดแล้ว กำลังแตกไฟล์
upload-already = มีอยู่ในคลังแล้ว
upload-stopped = หยุดแล้ว
upload-files-of = { $done } จาก { $total } ไฟล์
upload-resumed = อัปโหลดต่อ มีอยู่แล้ว { $size }
upload-skipped = ไฟล์ชุดเดียวกันนี้เคยนำเข้าแล้วในชื่อ “{ $name }” จึงข้ามไป
upload-error = { $error } ความคืบหน้าของคุณยังอยู่: ลองใหม่ หรือวางไฟล์ชุดเดิมอีกครั้งในภายหลัง
upload-retry = ลองใหม่
# $name is the pack the library has already that these files go into.
upload-update = อัปเดต “{ $name }”
upload-copy = นำเข้าเป็นสำเนา
# $name is the pack the library has already that the upload goes into.
upload-into = กำลังเข้าไปใน “{ $name }” ซึ่งมีอยู่ในคลังแล้ว ไฟล์ใหม่จะถูกเพิ่มและไฟล์ที่เปลี่ยนจะถูกแทนที่
upload-own-pack = แยกเป็นแพ็กของตัวเอง
upload-dismiss = ปิด
upload-cancel = ยกเลิก
upload-minimize = ย่อ
upload-expand = แสดงการนำเข้า
upload-pill = กำลังนำเข้า { $count } · { $percent }%
upload-pill-stopped = หยุดแล้ว: { $count }
upload-interrupted = ถูกขัดจังหวะ
upload-resume = ทำต่อ
upload-resume-all = ทำต่อทั้งหมด
upload-resume-hint-folder = เลือกโฟลเดอร์เดิมอีกครั้ง หรือลากมาวางบนหน้า เพื่อทำต่อจากจุดที่หยุดไว้
upload-resume-hint-file = เลือกไฟล์เดิมอีกครั้ง หรือลากมาวางบนหน้า เพื่อทำต่อจากจุดที่หยุดไว้
upload-not-same = ไม่ใช่ไฟล์ชุดเดิม จึงไม่มีอะไรทำต่อ
upload-permission-denied = เบราว์เซอร์ไม่ได้รับอนุญาตให้อ่านไฟล์อีกครั้ง โปรดเลือกไฟล์แทน

## PacksView.vue

packs-rename-failed = เปลี่ยนชื่อไม่ได้: { $error }
packs-collection-deleted = ลบคอลเลกชัน { $name } แล้ว
packs-delete-failed = ลบไม่ได้: { $error }
packs-status-uploading = กำลังอัปโหลด (ไฟล์ที่ได้รับจนถึงตอนนี้)
packs-status-ingesting = กำลังแตกไฟล์และติดแท็ก
packs-status-analyzing = LLM กำลังติดแท็ก
packs-status-ready = พร้อม
packs-status-failed = ล้มเหลว
packs-retag-failed = เริ่มติดแท็กใหม่ไม่ได้: { $error }
packs-read-files-failed = อ่านไฟล์ของแพ็กไม่ได้: { $error }
packs-context-load-failed = โหลดรายละเอียดร้านค้าของแพ็กไม่ได้: { $error }
packs-undo-store-failed = นำหน้าร้านค้าออกไม่ได้: { $error }
packs-context-save-failed = บันทึกรายละเอียดร้านค้าไม่ได้: { $error }
packs-unreal-load-failed = โหลดสถานะการแปลง Unreal ไม่ได้: { $error }
packs-unreal-discard-title = ทิ้งไฟล์ต้นฉบับหรือไม่?
packs-unreal-discard-body = ไฟล์ .uasset ต้นฉบับใน { $name } จะถูกลบเมื่อแปลงเสร็จ ข้อมูลเฉพาะของ Unreal ในไฟล์เหล่านั้น เช่น กราฟแมทีเรียล จะสูญหายถาวร
packs-unreal-discard-confirm = แปลงและทิ้ง
packs-unreal-convert-failed = เริ่มการแปลงไม่ได้: { $error }
packs-unreal-fact-converted = แปลงจาก Unreal แล้ว { $done }
packs-unreal-fact-packages = แพ็กเกจ Unreal { $left } รายการ
packs-unreal-fact-both = แปลงจาก Unreal แล้ว { $done } ยังเป็นแพ็กเกจอยู่ { $left }
packs-thumbs-canceled = ยกเลิกหลังจากทำไป { $done } จาก { $total }
packs-thumbs-drew = วาดโมเดลแล้ว { $count } รายการ
packs-thumbs-result = { $lead }: แทนที่ภาพย่อแล้ว { $improved } ภาพ
packs-thumbs-result-failed = { $lead }: แทนที่ภาพย่อแล้ว { $improved } ภาพ วาดไม่ได้ { $failed } ภาพ
packs-archived = เก็บถาวร { $name } และแอสเซ็ต { $count } รายการของแพ็กนี้แล้ว
packs-blobs-bad = มี { $count } ไฟล์ที่ไม่ผ่านการตรวจสอบความสมบูรณ์ครั้งล่าสุด
packs-collections = คอลเลกชัน
packs-browse-named = เรียกดู { $name }
packs-collection-name = ชื่อคอลเลกชัน
packs-save = บันทึก
packs-cancel = ยกเลิก
packs-assets = { $count } แอสเซ็ต
packs-updated = อัปเดตเมื่อ { $date }
packs-browse = เรียกดู
packs-collection-download = ดาวน์โหลด (.zip)
packs-collection-credits = เครดิต
packs-collection-credits-title = ผู้สร้างแพ็กที่แอสเซ็ตเหล่านี้มาจาก สัญญาอนุญาต และข้อความเครดิตที่ผู้สร้างขอ
packs-rename = เปลี่ยนชื่อ
packs-delete = ลบ
packs-packs = แพ็ก
packs-empty = ยังไม่มีแพ็ก วางไฟล์ zip หรือโฟลเดอร์ที่ใดก็ได้บนหน้านี้เพื่อเพิ่ม
packs-find = ค้นหาแพ็ก
packs-find-placeholder = ค้นหาแพ็กตามชื่อ ร้านค้า สัญญาอนุญาต หรือแท็ก
packs-sort = ลำดับ
packs-sort-new = ใหม่สุดก่อน
packs-sort-old = เก่าสุดก่อน
packs-sort-size = ใหญ่สุดก่อน
packs-sort-assets = แอสเซ็ตมากสุดก่อน
packs-sort-count = จำนวนแอสเซ็ต
packs-sort-smallest = เล็กสุดก่อน
packs-sort-fewest = แอสเซ็ตน้อยสุดก่อน
packs-with-kind = แพ็กที่มี
packs-state = สถานะ
packs-state-any = สถานะใดก็ได้
packs-state-working = กำลังดำเนินการ
packs-state-failed = ล้มเหลว
packs-state-untagged = ยังไม่มีคำอธิบาย
packs-state-unlicensed = ไม่ได้ระบุสัญญาอนุญาต
packs-store = ร้านค้า
packs-store-any = ร้านค้าใดก็ได้
packs-store-from = จาก { $store }
packs-store-none = ไม่มีหน้าร้านค้า
packs-shown-of = { $shown } จาก { $total }
packs-clear = ล้าง
packs-no-match = ไม่มีแพ็กที่ตรงกัน
packs-clear-all = ล้างการค้นหาและตัวกรอง
packs-pack-name = ชื่อแพ็ก
packs-thumbs-drawing = กำลังวาดภาพย่อในเบราว์เซอร์นี้…
packs-thumbs-drawing-count = กำลังวาดภาพย่อในเบราว์เซอร์นี้: { $done } จาก { $total }…
packs-thumbs-stopped = หยุดแล้ว: { $error }
packs-thumbs-none = ทุกโมเดลที่นี่มีภาพย่อที่วาดพร้อมเท็กซ์เจอร์ครบทั้งหมดแล้ว
packs-dismiss = ปิด
packs-license = สัญญาอนุญาต: { $license }
packs-license-tier = สัญญาอนุญาต: { $license } ({ $tier })
packs-license-not-found = ไม่พบ
packs-from-before = จาก
packs-from-after = { "" }
packs-added = เพิ่มเมื่อ { $date }
pack-panel-label = เกี่ยวกับแพ็กนี้
pack-panel-hide = ซ่อนรายละเอียดของแพ็ก
pack-panel-kinds = มีอะไรอยู่ข้างใน
pack-panel-all = แพ็กทั้งหมด
packs-download = ดาวน์โหลดแพ็ก (.zip)
packs-store-details = รายละเอียดร้านค้า
packs-more-named = การทำงานเพิ่มเติมสำหรับ { $name }
packs-more = เพิ่มเติม
packs-rename-more = เปลี่ยนชื่อ…
packs-retag-rules = เรียกใช้ตัวติดแท็กออฟไลน์อีกครั้ง
packs-retag-llm = เรียกใช้ตัวติดแท็ก LLM อีกครั้ง
packs-make-thumbs = สร้างภาพย่อในเบราว์เซอร์นี้
packs-make-thumbs-title = เปิดแต่ละโมเดลในตัวแสดงผล 3 มิติแบบซ่อนทีละรายการ แล้วใช้ภาพนั้นเป็นภาพย่อหากดีกว่าภาพของเซิร์ฟเวอร์
packs-make-thumbs-title-desktop = เปิดแต่ละโมเดลในตัวแสดงผล 3 มิติแบบซ่อนทีละรายการ แล้วใช้ภาพนั้นเป็นภาพย่อหากดีกว่าภาพที่ Hoard สร้าง
packs-read-files = อ่านไฟล์ของแพ็กอีกครั้ง
packs-credits = ดาวน์โหลดเครดิต
packs-credits-title = CREDITS.md: ผู้สร้างแพ็ก สัญญาอนุญาต และข้อความเครดิตที่ผู้สร้างขอ
packs-unreal-open = แปลงเนื้อหา Unreal…
packs-archive = เก็บถาวรแพ็ก
packs-unreal-intro = Hoard อ่านแพ็กเกจ Unreal ได้เองและจัดเก็บสิ่งที่อยู่ข้างในเป็นไฟล์ทั่วไป: เท็กซ์เจอร์เป็น PNG หรือ EXR เสียงเป็น WAV แมทีเรียลเป็น JSON ไฟล์ที่แปลงแล้วจะแทนที่แพ็กเกจ โดยคงแท็ก ลิงก์ และคอลเลกชันไว้
packs-unreal-converted =
    { $count ->
       *[other] แปลงแล้ว
    }
packs-unreal-converted-kept =
    { $count ->
       *[other] แปลงแล้ว เก็บต้นฉบับไว้ { $kept } รายการ
    }
packs-unreal-pending =
    { $count ->
       *[other] ยังไม่ได้ตรวจดู
    }
packs-unreal-needs-editor =
    { $count ->
       *[other] รอ Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
       *[other] ไม่มีรูปแบบทั่วไปที่รองรับ
    }
packs-unreal-failed =
    { $count ->
       *[other] ล้มเหลว
    }
packs-unreal-reasons = เหตุใดจึงมีแพ็กเกจเหลืออยู่
packs-unreal-editor-before = เมชและแอนิเมชันต้องใช้ Unreal Editor บนเครื่องที่ติดตั้ง Unreal ให้เรียกใช้
packs-unreal-editor-command = hoard unreal-worker --editor <พาธของ UnrealEditor-Cmd>
packs-unreal-editor-after = { " " }ซึ่งจะส่งออกเป็น FBX และอัปโหลดผลลัพธ์มาที่นี่
packs-unreal-discard = ทิ้งไฟล์ .uasset ต้นฉบับเมื่อแปลงเสร็จ
packs-unreal-discard-hint = คืนพื้นที่ ข้อมูลเฉพาะของ Unreal เช่น กราฟแมทีเรียล จะสูญหาย
packs-unreal-drop = ลบแพ็กเกจที่ไม่มีรูปแบบทั่วไปที่รองรับ
packs-unreal-drop-hint = Blueprint, เลเวล และแอสเซ็ตอื่น ๆ ที่ใช้ได้เฉพาะในเอนจิน
packs-unreal-convert = แปลง
packs-context-hint-before = แพ็กมาจากที่ใดและผู้ขายบอกอะไรเกี่ยวกับแพ็กนี้ ตัวติดแท็ก LLM จะอ่านข้อมูลนี้พร้อมทุกไฟล์ ดังนั้นคำโปรยจากร้านค้าอย่าง “120 ice and fire spells” จะช่วยให้ตั้งชื่อ
packs-context-hint-after = { " " }ได้
packs-store-auto = พบโดยอัตโนมัติ: แพ็กระบุหน้านี้ไว้เอง
packs-store-not-this = ไม่ใช่หน้านี้
packs-lookup-label = ค้นหาในร้านค้าด้วยคำว่า
packs-lookup-placeholder = ชื่อของแพ็กในร้านค้า
packs-lookup-searching = กำลังค้นหา…
packs-lookup-search = ค้นหาในร้านค้า
packs-lookup-hide = ซ่อน
packs-lookup-clues = จากไฟล์ของแพ็ก: { $clues }
packs-lookup-also = ค้นหา { $queries } ด้วยแล้ว
packs-lookup-quoted = “{ $query }”
packs-lookup-nothing = ไม่มีสิ่งใดบน Fab, itch.io, Kenney หรือ OpenGameArt ที่ดูคล้าย ลองใช้คำให้น้อยลงหรือคำอื่น
packs-lookup-use = ใช้
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = ตอนนี้
packs-review-store = จากร้านค้า
packs-review-same = เหมือนกัน
packs-review-read-page = จะอ่านจากหน้าเมื่อบันทึก
packs-review-take =
    { $count ->
       *[other] กรอก { $count } ช่อง
    }
packs-lookup-searched = ค้นหาเมื่อ { $date }
packs-lookup-again = ค้นหาอีกครั้ง
packs-lookup-again-after = { "" }
packs-lookup-unreachable = ครั้งนี้เชื่อมต่อ { $stores } ไม่ได้
packs-lookup-also-look = ลองดูที่
packs-lookup-links-and = { " " }และ{ " " }
packs-lookup-browser-only = (ค้นหาได้เฉพาะในเบราว์เซอร์ของคุณ) แล้ววางที่อยู่ของหน้าด้านล่าง
packs-lookup-open = ค้นหาในร้านค้า…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = ผู้ขาย
packs-seller-placeholder = ผู้สร้าง
packs-license-label = สัญญาอนุญาต
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = ระดับสัญญาอนุญาต
packs-license-tier-placeholder = Personal, Professional…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
       *[other] แอสเซ็ต { $count } รายการมีสัญญาอนุญาตของตัวเอง: ให้ใช้ตามสัญญาอนุญาตของแพ็กด้วย
    }
packs-store-page = หน้าร้านค้า
packs-store-text = คำอธิบายจากร้านค้า
packs-store-text-placeholder = วางข้อความในหน้าสินค้า หรือเว้นว่างไว้เพื่อดึงจากหน้าร้านค้า
packs-store-text-fetch = การบันทึกจะดึงคำอธิบายจากหน้าร้านค้า
packs-readme = README ที่พบในแพ็ก
packs-save-retag = บันทึกและเรียกใช้ตัวติดแท็ก LLM อีกครั้ง

## AssetDetail.vue

asset-lazy-3d-viewer = ตัวแสดงผล 3 มิติ
asset-lazy-text = ข้อความ
asset-lazy-text-viewer = ตัวแสดงข้อความ
asset-lazy-image-viewer = ตัวแสดงภาพ
asset-lazy-material-preview = ตัวอย่างแมทีเรียล
asset-lazy-material-viewer = ตัวแสดงแมทีเรียล
asset-lazy-sprite-player = ตัวเล่นสไปรต์
asset-lazy-sheet-view = ตัวแสดงแผ่นสไปรต์
asset-rel-texture = เท็กซ์เจอร์
asset-rel-buffer = ข้อมูลเรขาคณิต
asset-rel-material = ไฟล์แมทีเรียล
asset-rel-animation = แอนิเมชัน
asset-rel-library = ไฟล์ที่ลิงก์
asset-rel-guess = น่าจะใช้คู่กัน
asset-rel-sidecar = รายละเอียดร้านค้า
asset-rel-variant = รูปแบบย่อย
asset-rel-sprite = สไปรต์
asset-rel-sample = แซมเปิล
asset-role-map = แมป { $role }
asset-maps-count = { $count } แมป
asset-needs = ต้องใช้
asset-animations = แอนิเมชัน
asset-sprites = สไปรต์
asset-used-by = ถูกใช้โดย
asset-loose-sprites = มีเป็นไฟล์แยกด้วย
asset-in-sheet = อยู่ในสไปรต์ชีต
asset-other-formats = รูปแบบอื่น
asset-shown-as = แสดงในคลังเป็น
asset-conversion-failed = การแปลงล้มเหลว
asset-as-converting = กำลังแปลง…
asset-as-converting-percent = กำลังแปลง… { $percent }%
asset-as-queued = รอคิว…
asset-download = ดาวน์โหลด
asset-as-label = ดาวน์โหลดเป็น
asset-as-original = { $format } (รูปแบบต้นฉบับ)
asset-as-pack-files = ไฟล์ของแพ็กเอง
asset-as-size = ขนาด
asset-as-full-size = ขนาดเต็ม
asset-as-up-to = ไม่เกิน { $size } px
asset-as-hint = เลือกขนาดหรือรูปแบบอื่น
asset-meta-size = ขนาด
asset-meta-length = ความยาว
asset-meta-sample-rate = อัตราสุ่มตัวอย่าง
asset-meta-channels = ช่องสัญญาณ
asset-meta-mono = โมโน
asset-meta-stereo = สเตอริโอ
asset-meta-bit-depth = ความลึกบิต
asset-meta-bits = { $bits } บิต
asset-meta-level = ระดับเสียง
asset-meta-level-value = พีก { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = พีก { $peak } dBFS, RMS { $rms } dBFS, มีการคลิป
asset-meta-loop = ลูป
asset-meta-loop-value = { $start } ถึง { $end }
asset-meta-playback = การเล่น
asset-meta-one-shot = เล่นครั้งเดียว
asset-meta-envelope = เอนเวโลป
asset-meta-envelope-times = ช่วงเริ่ม { $attack }, ช่วงท้าย { $tail }
asset-meta-envelope-hit = กระแทก
asset-meta-envelope-swell = ค่อยๆ ดังขึ้น
asset-meta-envelope-sustained = คงที่
asset-meta-tone = โทนเสียง
asset-meta-tone-value = ความสว่าง { $khz } kHz, เสียงรบกวน { $noise }%
asset-meta-pitch = ระดับเสียง
asset-meta-pitch-rises = สูงขึ้น { $octaves } ออกเทฟ
asset-meta-pitch-falls = ต่ำลง { $octaves } ออกเทฟ
asset-meta-loops-cleanly = วนซ้ำได้อย่างไร้รอยต่อ
asset-meta-tempo = จังหวะ
asset-meta-tempo-heard = ประมาณ { $bpm } BPM จากการฟัง
asset-meta-tempo-heard-or = ประมาณ { $bpm } BPM (หรือ { $alt }) จากการฟัง
asset-meta-key = คีย์
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key } จากการฟัง
music-key-major = { $note } เมเจอร์
music-key-minor = { $note } ไมเนอร์
asset-meta-root-note = โน้ตหลัก
asset-meta-cue-markers = เครื่องหมายคิว
asset-meta-library = ไลบรารี
asset-meta-keywords = คำสำคัญ
asset-meta-title = ชื่อเรื่อง
asset-meta-artist = ศิลปิน
asset-meta-album = อัลบั้ม
asset-meta-genre = แนว
asset-meta-generator = ตัวสร้าง
asset-meta-everything-with = แสดงทุกอย่างที่มีแท็ก { $tag }
asset-meta-saved-with = บันทึกด้วย
asset-meta-color-mode = โหมดสี
asset-meta-layers = เลเยอร์
asset-meta-range = ช่วง
asset-meta-hdr = HDR (ตัวอย่างผ่านการปรับโทน)
asset-meta-format = รูปแบบ
asset-meta-large-document = เอกสารขนาดใหญ่ (PSB)
asset-meta-ai = AI
asset-meta-ai-made = สร้างด้วย AI แบบรู้สร้าง
asset-meta-ai-edited = แก้ไขด้วย AI แบบรู้สร้าง
asset-meta-ai-how-metadata = เมทาดาทา
asset-meta-ai-how-generator = การตั้งค่าตัวสร้าง
asset-meta-ai-how-iptc = ประเภทแหล่งที่มาดิจิทัลของ IPTC
asset-meta-ai-says = { $made } ตามที่ระบุใน{ $how }
asset-meta-ai-by-says = { $made } ({ $by }) ตามที่ระบุใน{ $how }
asset-meta-provenance = ที่มา
asset-meta-c2pa = มี C2PA Content Credentials
asset-meta-compression = การบีบอัด
asset-meta-note = โน้ต
asset-meta-maps = แมป
asset-meta-sizes =
    { $count ->
       *[other] ขนาด
    }
asset-meta-triangles = สามเหลี่ยม
asset-meta-vertices = จุดยอด
asset-meta-meshes = เมช
asset-meta-materials = แมทีเรียล
asset-meta-animations = แอนิเมชัน
asset-meta-skins = สกิน
asset-meta-unreal-asset = แอสเซ็ต Unreal
asset-meta-unreal-saved-with = { $class } บันทึกด้วย UE { $engine }
asset-meta-package = แพ็กเกจ
asset-meta-cooked = ผ่านการ cook (บิลด์เกม): ตัวอย่างและซอร์สถูกตัดออก
asset-meta-tiling = การเรียงต่อ
asset-meta-seamless = ต่อกันได้ไร้รอยต่อทั้งสองแกน
asset-meta-analysis-error = อ่านไฟล์นี้ได้ไม่ครบ: { $error }
asset-copy-blocked = คัดลอกไม่ได้: เบราว์เซอร์บล็อกไว้
asset-save-error = บันทึกไม่ได้: { $error }
asset-tag-removed = นำ { $tag } ออกแล้ว
asset-tag-restored = ใส่ { $tag } กลับแล้ว
asset-tags-added = เพิ่มแท็กแล้ว { $count } รายการ
asset-tags-accepted = เก็บแท็กที่เดาไว้ { $count } แท็ก
asset-reviewed = ตรวจแล้ว
asset-reviewed-on-title = ตรวจแล้ว: กฎและ AI จะไม่แตะแท็กของแอสเซ็ตนี้ คลิกเพื่อเอาเครื่องหมายออก (R)
asset-reviewed-off-title = ทำเครื่องหมายว่าตรวจแล้ว กฎและ AI จะได้ไม่แตะแท็กของแอสเซ็ตนี้ (R)
asset-reviewed-on = ทำเครื่องหมายว่าตรวจแล้ว
asset-reviewed-off = เอาเครื่องหมายตรวจแล้วออก
# $name is an asset's name.
asset-reviewed-on-named = ทำเครื่องหมาย { $name } ว่าตรวจแล้ว
asset-reviewed-off-named = เอาเครื่องหมายตรวจแล้วออกจาก { $name }
asset-undo = เลิกทำ
asset-play-error = เล่นไฟล์นี้ไม่ได้: { $error }
asset-copied-hash = คัดลอกแฮชแล้ว
asset-copied-toml = คัดลอก toml แล้ว
asset-copied-id = คัดลอก ID แล้ว
asset-copied-link = คัดลอกลิงก์แล้ว
asset-origin-before = ไฟล์นี้เดิมคือ
asset-origin-class-engine = { " " }ซึ่งเป็น { $class } ที่บันทึกโดย UE { $engine }
asset-origin-class = { " " }ซึ่งเป็น { $class }
asset-origin-engine = { " " }ที่บันทึกโดย UE { $engine }
asset-origin-end = { "" }
asset-origin-editor = Unreal Editor ส่งออกเมื่อ { $date }
asset-origin-editor-notes = Unreal Editor ส่งออกเมื่อ { $date } ({ $notes })
asset-origin-hoard = Hoard อ่านออกมาจากแพ็กเกจเมื่อ { $date }
asset-origin-hoard-notes = Hoard อ่านออกมาจากแพ็กเกจเมื่อ { $date } ({ $notes })
asset-origin-download = ดาวน์โหลดไฟล์ .{ $ext } ต้นฉบับ
asset-origin-discarded = แพ็กเกจต้นฉบับถูกทิ้งไปหลังการแปลง
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = ประวัติ
history-loading = กำลังโหลด…
history-none = ยังไม่มีการเปลี่ยนแปลงที่นี่
history-load-failed = โหลดประวัติไม่ได้: { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = AI
history-by-rule = กฎ
history-by-someone = บางคน
history-by-app = { $who } ผ่านแอป
history-by-assistant = { $who } ผ่านผู้ช่วย
history-field-description = คำอธิบาย
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = เพิ่มแท็ก { $tag }
history-tag-removed = นำแท็ก { $tag } ออก
history-tag-kept = เก็บแท็กที่แนะนำ { $tag } ไว้
history-tag-changed = เปลี่ยนแท็ก { $tag }
history-reviewed = ทำเครื่องหมายว่าตรวจแล้ว
history-unreviewed = ทำเครื่องหมายว่ายังไม่ตรวจ
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field }: ตั้งเป็น { $to }
history-cleared = { $field }: ล้างแล้ว (เดิมคือ { $from })
history-changed = { $field }: { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = เลิกทำ
history-revert-title = คืนค่าตามก่อนการเปลี่ยนแปลงนี้
history-reverted = เลิกทำแล้ว: { $change }
history-revert-failed = เลิกทำไม่ได้: { $error }
asset-details = รายละเอียดแอสเซ็ต
asset-in-pack = ใน
asset-previous = แอสเซ็ตก่อนหน้า
asset-next = แอสเซ็ตถัดไป
asset-favorite = รายการโปรด
asset-favorite-on-title = อยู่ในรายการโปรดของคุณ (กด F เพื่อนำออก)
asset-favorite-off-title = เพิ่มในรายการโปรด (F)
asset-share = แชร์
asset-share-title = แชร์ลิงก์ไปยังแอสเซ็ตนี้ (ผู้ที่เปิดจะต้องลงชื่อเข้าใช้ Hoard ก่อน)
asset-layout-panel = แสดงในแผงด้านข้าง
asset-layout-page = แสดงเต็มหน้า
asset-layout-page-title = แสดงเต็มหน้า: ตัวอย่างขนาดใหญ่ข้างรายละเอียด
asset-close = ปิดรายละเอียด
asset-rename-title = คลิกเพื่อเปลี่ยนชื่อ
asset-name = ชื่อ
asset-views-alt = โมเดลจากสี่ทิศทาง
asset-inspect-aria = ตรวจดู { $name } แบบเต็มจอ
asset-inspect-title = ตรวจดู: ซูม เรียงต่อ ช่องสี
asset-playback-position = ตำแหน่งการเล่น
asset-playback-value = { $current } จาก { $duration }
asset-pause = หยุดชั่วคราว
asset-play = เล่น
asset-waveform-drawing = กำลังวาดรูปคลื่น…
asset-waveform-skipped = ไม่มีรูปคลื่นสำหรับไฟล์ขนาดใหญ่เท่านี้
asset-play-animation = เล่นเป็นแอนิเมชัน
asset-show-views = แสดงแผ่นภาพสี่มุมมองที่ตัวติดแท็กเห็น
asset-palette = สีในภาพ
asset-swatch-title = { $hex } คิดเป็น { $percent }% ของภาพ ค้นหาแอสเซ็ตที่มีสีนี้
asset-swatch-aria = ค้นหาแอสเซ็ตที่มี { $hex } คิดเป็น { $percent }% ของภาพนี้
asset-more-like-this = เพิ่มเติมที่คล้ายกัน
asset-desc-label = คำอธิบายของคุณ
asset-desc-placeholder = สิ่งนี้คืออะไร คุณจะใช้ที่ไหน…
asset-save = บันทึก
asset-cancel = ยกเลิก
asset-desc-family = บันทึกให้รูปแบบย่อยทั้ง { $count } รายการ
asset-desc-edit = แก้ไขคำอธิบาย
asset-desc-write = เขียนคำอธิบาย
asset-desc-ai = เขียนโดย AI
asset-desc-file = จากไฟล์
asset-desc-store-before = จาก
asset-desc-store-page = หน้า { $store }
asset-desc-store-page-unnamed = หน้าร้านค้า
asset-desc-store-after = (ทั้งแพ็ก)
asset-less = น้อยลง
asset-more = เพิ่มเติม
asset-tags = แท็ก
asset-tag-add-label = เพิ่มแท็ก
asset-tag-add-placeholder = เพิ่มแท็ก เช่น mood:tense
asset-apply-family = ใช้การเปลี่ยนแปลงกับรูปแบบย่อยทั้ง { $count } รายการ
asset-tags-accept-all = เก็บแท็กที่เดาทั้ง { $count } แท็ก
asset-tags-accept-all-title = เก็บทุกแท็กที่กฎและ AI เดาไว้เป็นแท็กของคุณเอง
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = ที่ปฏิเสธไว้:
asset-tag-restore-title = ใส่ { $tag } กลับ
asset-tags-also-under = อยู่ภายใต้ด้วย:
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = รายการที่คล้ายกันมี:
asset-tag-suggest-title = เพิ่ม { $tag }
asset-details-heading = รายละเอียด
asset-field-category = หมวดหมู่
asset-field-license = สัญญาอนุญาต
asset-field-source = แหล่งที่มา
asset-field-rating = คะแนน
# Shown in place of an empty value, to fill it in.
asset-field-add = เพิ่ม
asset-field-new = เพิ่มช่อง
asset-field-name = ชื่อช่อง
asset-field-value = ค่า
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = จากแพ็ก
asset-field-license-placeholder = เช่น CC0 หรือ CC-BY 4.0
asset-field-remove-placeholder = เว้นว่างเพื่อลบ
# $field is a field's name, like "License".
asset-field-edit-title = แก้ไข { $field }
asset-field-saved = บันทึก { $field } แล้ว
asset-field-cleared = ล้าง { $field } แล้ว
asset-rate = { $count } ดาว
asset-rated = ให้ { $count } ดาว
asset-rating-clear-title = คลิกอีกครั้งเพื่อล้างคะแนน
asset-rating-cleared = ล้างคะแนนแล้ว
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
       *[other] คลิกอีกครั้งเพื่อให้ { $count } ดาว
    }
asset-collections = คอลเลกชัน
asset-collection-remove = นำออกจาก { $name }
asset-collection-add-another = เพิ่มลงในคอลเลกชันอื่น
asset-collection-add = เพิ่มลงในคอลเลกชัน
asset-used-in = ใช้อยู่ใน
asset-used-in-title = แสดงสิ่งที่ { $name } มี
asset-used-in-synced = ซิงก์เมื่อ { $when }
asset-maps = แมป
asset-show-more = แสดงเพิ่มอีก { $count } รายการ
asset-materials-heading =
    { $count ->
       *[other] แมทีเรียล
    }
asset-material-maps-aria = { $name }: แมป
asset-variants = รูปแบบย่อย ({ $count } เวอร์ชัน)
asset-fit-animations = แอนิเมชันที่ใช้ได้ ({ $count })
asset-fit-characters = ตัวละครที่ใช้ได้ ({ $count })
asset-copies = สำเนาที่เหมือนกันทุกประการ
asset-copies-note = ข้อมูลชุดเดียวกันจัดเก็บเพียงครั้งเดียว แพ็กเหล่านี้ก็มีไฟล์นี้เช่นกัน
asset-near-copies = สำเนาที่เกือบเหมือนกัน
asset-near-copies-note = สิ่งเดียวกันในขนาด อัตราสุ่มตัวอย่าง หรือรูปแบบอื่น สำเนาที่ดีที่สุดมีเครื่องหมายไว้
asset-near-copies-best = สิ่งเดียวกันในขนาด อัตราสุ่มตัวอย่าง หรือรูปแบบอื่น ไฟล์นี้คือสำเนาที่ดีที่สุด
asset-near-best = ดีที่สุด
asset-unreal-converted = แปลงจาก Unreal
asset-rule-converted = แปลงโดยกฎ
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = กฎการแปลงสร้าง { $format } นี้เมื่อ { $date } จาก
asset-rule-kept-until = เก็บต้นฉบับไว้จนถึง { $date }
asset-rule-rebuilt = FLAC เก็บทุกอย่างที่อยู่ใน WAV ไว้ จึงดาวน์โหลดหรือนำ WAV กลับมาได้ตรงตามเดิมทุกเมื่อ
asset-rule-put-back = นำต้นฉบับกลับมา
asset-rule-put-back-title = นำ { $file } กลับมาหรือไม่?
asset-rule-put-back-body = ไฟล์ที่แปลงแล้วจะถูกลบ และต้นฉบับจะกลับมาแทนที่ พร้อมแท็ก คอลเลกชัน และลิงก์ของไฟล์ หลังจากนี้กฎการแปลงจะไม่แตะต้องไฟล์นี้อีก
asset-rule-put-back-done = นำ { $file } กลับมาแล้ว
asset-rule-let-go = ต้นฉบับถูกลบไปแล้วเมื่อครบระยะเวลาเก็บต้นฉบับที่กำหนดไว้ในการตั้งค่า
asset-unreal-package = จากแพ็กเกจ Unreal
asset-ue-needs-editor-before = กำลังรอ Unreal Editor: { $reason } บนเครื่องที่ติดตั้ง Unreal ให้เรียกใช้
asset-ue-needs-editor-after = แล้วจะได้กลับมาเป็น FBX
asset-ue-editor-path = พาธของ UnrealEditor-Cmd
asset-ue-unsupported = เก็บไว้เป็นแพ็กเกจ: { $reason }
asset-ue-failed = การแปลงล้มเหลว: { $reason }
asset-recovered = ไฟล์ .{ $ext } ต้นฉบับถูกเก็บไว้ในแพ็กเกจนี้ และ Hoard ดึงออกมาให้แล้ว
asset-recovered-download = ดาวน์โหลด .{ $ext }
asset-ue-meshes-before = เมชและแอนิเมชันจัดเก็บในรูปแบบของ Unreal เอง ให้แปลงแพ็ก (หน้าแพ็ก, เพิ่มเติม) แล้วเรียกใช้
asset-ue-meshes-after = บนเครื่องที่ติดตั้ง Unreal Editor เพื่อให้ได้ FBX
asset-ue-registry-tags = แท็ก asset registry { $count } รายการ
asset-file = ไฟล์
asset-bytes = ไบต์
asset-source = แหล่งที่มา
asset-path = พาธ
asset-path-open-folder = ดูว่ามีอะไรอีกใน { $folder }
asset-download-maps = ดาวน์โหลดแมปทั้งหมด { $count } แมป
asset-download-linked = ดาวน์โหลดพร้อมไฟล์ที่ลิงก์ { $count } ไฟล์
asset-download-only-title = เฉพาะไฟล์นี้ โดยไม่รวมไฟล์ที่ไฟล์นี้ใช้
asset-download-only = เฉพาะไฟล์นี้
asset-include-animations = รวมแอนิเมชันไว้ใน .zip
asset-include-character = รวมตัวละคร { $name }
asset-copy-toml = คัดลอกรายการ hoard.toml
asset-copy-id = คัดลอก ID
asset-teach = สอนให้ Hoard รู้จักไฟล์นี้
asset-teach-title = อธิบายไฟล์ของร้านค้านี้เพื่อให้ Hoard อ่านได้ (การตั้งค่า → ไฟล์ร้านค้า)
asset-archive = เก็บถาวร
asset-archive-title = ไม่แสดงในการค้นหาจนกว่าจะกู้คืนหรือลบถาวรจากที่เก็บถาวร

## ImageViewer.vue

image-channel-red = ช่องสีแดง
image-channel-green = ช่องสีเขียว
image-channel-blue = ช่องสีน้ำเงิน
image-channel-alpha = ช่องอัลฟา
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = พื้นหลังลายตาราง (K เพื่อสลับ)
image-bg-black-title = พื้นหลังสีดำ (K เพื่อสลับ)
image-bg-gray-title = พื้นหลังสีเทา (K เพื่อสลับ)
image-bg-white-title = พื้นหลังสีขาว (K เพื่อสลับ)
image-bg-checker = พื้นหลังลายตาราง
image-bg-black = พื้นหลังสีดำ
image-bg-gray = พื้นหลังสีเทา
image-bg-white = พื้นหลังสีขาว
image-inspect-aria = ตรวจดู { $name }
image-preview-size = ตัวอย่าง { $width }×{ $height }, { $bytes }
image-preview = ตัวอย่าง
image-resolution = ความละเอียด
image-full-title = โหลดภาพเต็ม { $width }×{ $height } (F) การซูมเกินขนาดตัวอย่างก็จะโหลดเช่นกัน
image-full-loading = กำลังโหลดความละเอียดเต็ม…
image-full-failed = โหลดความละเอียดเต็มไม่สำเร็จ: ลองใหม่
image-full = ความละเอียดเต็ม
image-zoom = ซูม
image-zoom-out-title = ซูมออก (−)
image-zoom-out = ซูมออก
image-zoom-in-title = ซูมเข้า (+)
image-zoom-in = ซูมเข้า
image-fit-title = พอดี (0)
image-fit = พอดี
image-actual-title = พิกเซลจริง (1)
image-view = มุมมอง
image-tile-title = เรียงต่อ 3×3 เพื่อตรวจรอยต่อ (T)
image-tile = เรียงต่อ
image-channels = ช่องสี
image-close = ปิด (Esc)
image-failed = โหลดภาพนี้ไม่ได้
image-loading = กำลังโหลด…
image-key-gestures = เลื่อนหรือจีบนิ้วเพื่อซูม · ลากเพื่อเลื่อนภาพ · ดับเบิลคลิกเพื่อซูมเข้าหรือพอดี
image-key-tile = เรียงต่อ
image-key-background = พื้นหลัง
image-key-full = ความละเอียดเต็ม
image-key-channels = ช่องสี

## SpritePlayer.vue

sprite-frame-aria = { $name } เฟรม { $frame } จาก { $count }
sprite-failed = เล่นไฟล์นี้ไม่ได้
sprite-loading = กำลังโหลดเฟรม…
sprite-previous = เฟรมก่อนหน้า
sprite-next = เฟรมถัดไป
sprite-pause = หยุดแอนิเมชันชั่วคราว
sprite-play = เล่นแอนิเมชัน
sprite-animation = แอนิเมชัน
sprite-all-frames = ทุกเฟรม
sprite-grid-title = วิธีตัดแผ่นภาพเป็นเฟรม
sprite-columns = คอลัมน์
sprite-rows = แถว
sprite-fps-aria = เฟรมต่อวินาที
sprite-fps = fps

## The sprite sheet viewer (SheetView.vue) and its download formats (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
       *[other] { $count } สไปรต์
    }
sheet-pages =
    { $count ->
       *[other] { $count } หน้า
    }
sheet-animations =
    { $count ->
       *[other] { $count } แอนิเมชัน
    }
# The box that narrows the list of sprites by name.
sheet-find = ค้นหาสไปรต์
# Under the list of sprites when more match than it draws.
sheet-showing = แสดง { $shown } จาก { $total }
sheet-no-match = ไม่มีสไปรต์ที่มีคำนี้ในชื่อ
sheet-failed = อ่านแผ่นภาพนี้ไม่ได้
sheet-loading = กำลังโหลดแผ่นภาพ…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = รูปแบบของ Hoard เอง: ทุกหน้าคงไว้ตามเดิม สไปรต์หมุนได้ทุกทิศและตัดขอบโปร่งใสออก พร้อมแอนิเมชันและเวลาของแต่ละเฟรม
sheet-note-texturepacker = สไปรต์หมุนตามเข็มนาฬิกาและตัดขอบโปร่งใสออก พร้อมแอนิเมชันตามชื่อ ในแบบที่ PixiJS และ Phaser อ่านได้
sheet-note-clockwise = สไปรต์หมุนตามเข็มนาฬิกาและตัดขอบโปร่งใสออก รูปแบบนี้เก็บแอนิเมชันไม่ได้ เฟรมจึงเรียงตามชื่อที่มีหมายเลข
sheet-note-libgdx = สไปรต์หมุนทวนเข็มนาฬิกาและตัดขอบโปร่งใสออก รูปแบบนี้เก็บแอนิเมชันไม่ได้ เฟรมจึงเรียงตามชื่อที่มีหมายเลข
sheet-note-godot-frames = สไปรต์ไม่ถูกหมุนเลย ส่วนที่ตัดออกเก็บไว้เป็นระยะขอบ พร้อมแอนิเมชันและเวลาของแต่ละเฟรม
sheet-note-godot-tileset = ไทล์ไม่ถูกหมุนหรือตัดขอบ วางบนกริดตามขนาดไทล์ และคงภาพเดิมไว้หากเรียงเป็นกริดอยู่แล้ว
sheet-note-grid = สไปรต์ทุกตัวตั้งตรงบนผืนภาพเต็มของตัวเอง ในช่องขนาดเท่ากันทั้งหมด สำหรับโปรแกรมใดก็ตามที่ตัดแผ่นภาพเป็นกริด
sheet-note-frames = สไปรต์ทุกตัวตั้งตรงบนผืนภาพเต็มของตัวเอง แต่ละตัวเป็นไฟล์ PNG ที่ตั้งชื่อตามสไปรต์นั้น

## TextViewer.vue, TextPreview.vue และ text.ts

text-lang-config = การกำหนดค่า
text-lang-table = ตาราง
text-lang-code = โค้ด
text-lang-script = สคริปต์
text-lang-text = ข้อความ
text-copy-blocked = คัดลอกไม่ได้: เบราว์เซอร์บล็อกไว้
text-read-aria = อ่าน { $name }
text-lines = { $count } บรรทัด
text-show-as = แสดงเป็น
text-tree-title = ทรีที่พับได้ (T)
text-tree = ทรี
text-pretty-title = จัดย่อหน้าใหม่ (P)
text-pretty = จัดรูปแบบ
text-raw-title = ตามที่เขียนไว้ (R)
text-raw = ดิบ
text-document-title = เป็นเอกสาร (D)
text-document = เอกสาร
text-source-title = markdown ตามที่เขียนไว้ (R)
text-source = ซอร์ส
text-page-title = อย่างที่เบราว์เซอร์แสดงจากโฟลเดอร์ของมัน แยกออกจาก Hoard (P)
text-page = หน้าเว็บ
text-page-source-title = หน้าเว็บตามที่เขียนไว้ (R)
text-page-aria = { $name } แบบหน้าเว็บ
text-page-error = เปิดหน้านี้ไม่ได้: { $error }
text-fold = พับ
text-expand-all-title = คลี่ทุกส่วน
text-expand-all = ขยายทั้งหมด
text-collapse-all-title = พับทุกส่วน
text-collapse-all = ยุบทั้งหมด
text-group-text = ข้อความ
text-wrap-title = ตัดบรรทัดยาว (W)
text-wrap = ตัดบรรทัด
text-copy-title = คัดลอกข้อความ (C)
text-copied = คัดลอกแล้ว
text-copy = คัดลอก
text-download-title = ดาวน์โหลดไฟล์
text-download = ดาวน์โหลด
text-close = ปิด (Esc)
text-truncated = แสดง { $shown } แรกจากทั้งหมด { $total }
text-truncated-json = แสดง { $shown } แรกจากทั้งหมด { $total } (ทรีต้องใช้ทั้งไฟล์)
text-show-all = แสดงทั้งหมด
text-json-error = JSON นี้แยกวิเคราะห์ไม่ได้: { $error }
text-contents-aria = เนื้อหาของ { $name }
text-loading = กำลังโหลด…
text-load-error = โหลดไฟล์นี้ไม่ได้: { $error }
text-binary = ไฟล์นี้เป็นไบนารี ไม่ใช่ข้อความ จึงไม่มีอะไรให้อ่านที่นี่ ดาวน์โหลดเพื่อเปิดในเครื่องมือของไฟล์นั้น
text-key-tree = ทรี
text-key-pretty = จัดรูปแบบ
text-key-raw = ดิบ
text-key-document = เอกสาร
text-key-source = ซอร์ส
text-key-page = หน้าเว็บ
text-key-wrap = ตัดบรรทัด
text-key-copy = คัดลอก
text-key-find = ค้นหา
text-key-close = ปิด
text-preview-aria = อ่าน { $name } แบบเต็มจอ
text-preview-read = อ่าน

## JsonTree.vue และ JsonPicker.vue

json-items = { $count } รายการ
json-keys = { $count } คีย์
json-fold = พับ
json-unfold = คลี่
json-unfold-summary = คลี่ { $summary }
json-show-more = แสดงเพิ่มอีก { $count }
json-left = จากที่เหลือ { $count }
json-picker-more = … อีก { $count }

## ตัวแสดงผลโมเดล 3 มิติ (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = ตัวแสดงผล 3 มิติโหลดไม่สำเร็จ ({ $error }) การโหลดหน้าใหม่อาจช่วยได้
model-loading-viewer = กำลังโหลดตัวแสดงผล 3 มิติ…
model-loading = กำลังโหลดโมเดล…
model-show-failed = แสดงโมเดลนี้ไม่ได้: { $error }
model-preview-failed = แสดงตัวอย่างโมเดลนี้ไม่ได้: { $error }
model-error-converting = เซิร์ฟเวอร์ยังแปลงโมเดลนี้อยู่
model-error-converting-desktop = Hoard ยังแปลงโมเดลนี้อยู่
model-error-empty-collada = ไฟล์ Collada ว่างเปล่า
model-error-no-loader = เบราว์เซอร์ไม่มีตัวโหลดสำหรับ .{ $ext }
model-dialog = ดู { $name } แบบ 3 มิติ
model-title-stats = { $triangles } สามเหลี่ยม · { $meshes } เมช · { $materials } แมทีเรียล
model-triangles = { $count } สามเหลี่ยม
model-group-camera = กล้อง
model-group-shading = การแรเงา
model-group-background = พื้นหลัง
model-group-animation = แอนิเมชัน
model-group-view = มุมมอง
model-group-show = แสดง
model-frame-title = จัดโมเดลให้พอดีกรอบ (F)
model-frame = จัดกรอบ
model-reset-title = กลับไปที่มุมมองเริ่มต้น (R)
model-reset = รีเซ็ต
model-turn-title = หมุนช้า ๆ (A)
model-turn = หมุน
model-textures-title = เท็กซ์เจอร์ (T)
model-textures = เท็กซ์เจอร์
model-full-res-title = เท็กซ์เจอร์ความละเอียดเต็ม (H): ใช้ไฟล์เท็กซ์เจอร์ตามจริงไม่ว่าจะใหญ่แค่ไหน เมื่อปิดจะใช้ตัวอย่างแบบเบาขนาดไม่เกิน 2048 px
model-full-res = เต็ม
model-light-title = แสงสภาพแวดล้อม (E)
model-light = แสง
model-grid-title = ตารางพื้น (G)
model-grid = ตาราง
model-bg-checker = พื้นหลังลายตาราง
model-bg-dark = พื้นหลังมืด
model-bg-gray = พื้นหลังสีเทา
model-bg-light = พื้นหลังสว่าง
model-bg-env = พื้นหลังสภาพแวดล้อม
model-bg-title = { $background } (K เพื่อสลับ)
model-pause-title = หยุดชั่วคราว (Space)
model-play-title = เล่น (Space)
model-pause = หยุดชั่วคราว
model-play = เล่น
model-animation-clip = คลิปแอนิเมชัน
model-animation = แอนิเมชัน
model-plays-on = เล่นบนตัวละคร
model-plays-everywhere = ใช้ทุกที่
model-plays-everywhere-title = เล่นทุกแอนิเมชันของโครงกระดูกนี้บน { $name } รวมถึงในภาพขนาดย่อด้วย
model-plays-everywhere-done = แอนิเมชันของโครงกระดูกนี้เล่นบน { $name } แล้ว กำลังวาดภาพขนาดย่อใหม่
model-plays-everywhere-failed = เปลี่ยนตัวละครที่ใช้เล่นไม่ได้: { $error }
model-rest-pose = ท่าพัก
model-close-title = ปิด (Esc)
model-full-screen = เต็มจอ
model-open-full-screen = เปิดตัวแสดงผล 3 มิติแบบเต็มจอ
model-menu-fold = พับเมนูเก็บ
model-menu-show = แสดงเมนู
model-view-prefix = มุมมอง:{ " " }
model-menu-render = เรนเดอร์
model-menu-maps = แมป
model-menu-inspect = ตรวจดู
model-mode-final = เรนเดอร์สุดท้าย
model-mode-raw = เรนเดอร์ดิบ
model-mode-color = สีพื้นฐาน
model-mode-roughness = ความหยาบ
model-mode-metalness = ความเป็นโลหะ
model-mode-normal = นอร์มอล
model-mode-ao = อักคลูชัน
model-mode-matcap = Matcap
model-mode-uv = ตัวตรวจ UV
model-mode-geometry = รูปทรงเรขาคณิต
model-mode-title = { $mode } ({ $key })
model-wireframe-title = โครงลวด (W) ซ้อนบนมุมมองใดก็ได้
model-wireframe = โครงลวด
model-info-title = ข้อมูลฉาก (I)
model-info = ข้อมูลฉาก
model-info-close = ปิดข้อมูลฉาก
model-info-close-title = ปิด (I)
model-info-geometries = รูปทรงเรขาคณิต
model-info-materials = แมทีเรียล
model-info-textures = เท็กซ์เจอร์
model-info-triangles = สามเหลี่ยม
model-info-vertices = จุดยอด
model-info-meters = glTF ใช้หน่วยเมตร
model-info-no-units = รูปแบบนี้ไม่ได้ระบุหน่วย
model-info-height = ความสูง
model-info-length = ความยาว
model-info-width = ความกว้าง
model-length-units =
    { $n ->
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } หน่วย
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = ไม่พบเท็กซ์เจอร์: { $name }
model-missing-some = ไม่พบเท็กซ์เจอร์ { $count } ไฟล์: { $names }
model-missing-more = ไม่พบเท็กซ์เจอร์ { $count } ไฟล์: { $names }…
model-missing-count = ไม่พบเท็กซ์เจอร์ { $count } ไฟล์
model-guessed = จับคู่เท็กซ์เจอร์ตามชื่อ เพราะไฟล์ไม่ได้ระบุไว้
model-group-facing = ทิศทาง
model-facing-turn = หมุน 90°
model-facing-turn-title = หมุนโมเดลทวนเข็มนาฬิกา 90° เมื่อมองจากด้านบน
model-facing-tip = เอียง 90°
model-facing-tip-title = เอียงด้านบนของโมเดลไปทางด้านหน้า 90° เพื่อตั้งโมเดลที่นอนอยู่ให้ยืนขึ้น
model-facing-save = ใช้กับแพ็กนี้
model-facing-save-title = หมุนทุกโมเดลใน { $pack } แบบนี้ ภาพย่อจะถูกวาดใหม่ ส่วนไฟล์ดาวน์โหลดยังคงเหมือนตอนที่สร้างไว้
model-facing-undo = เลิกทำ
model-facing-undo-title = กลับไปเป็นการหมุนเดิมของโมเดลในแพ็ก
model-facing-saved = ตอนนี้ทุกโมเดลใน { $pack } หันไปทางนี้แล้ว กำลังวาดภาพย่อใหม่
model-facing-failed = หมุนโมเดลของแพ็กไม่ได้: { $error }
model-keys-mouse = ลากเพื่อหมุนรอบ · ลากด้วยปุ่มขวาเพื่อเลื่อน · เลื่อนล้อเพื่อซูม · ดับเบิลคลิกเพื่อจัดกรอบ
model-key-frame = จัดกรอบ
model-key-reset = รีเซ็ต
model-key-view = มุมมอง
model-key-wireframe = โครงลวด
model-key-info = ข้อมูลฉาก
model-key-textures = เท็กซ์เจอร์
model-key-full-res = เท็กซ์เจอร์ความละเอียดเต็ม
model-key-light = แสง
model-key-grid = ตาราง
model-key-background = พื้นหลัง
model-key-play = เล่น

## แมทีเรียล (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = สี
material-role-normal = นอร์มอล
material-role-normal-dx = นอร์มอล (DirectX)
material-role-roughness = ความหยาบ
material-role-gloss = ความเงา
material-role-metalness = ความเป็นโลหะ
material-role-ao = แอมเบียนต์อักคลูชัน
material-role-orm = อักคลูชัน ความหยาบ โลหะ
material-role-mra = โลหะ ความหยาบ อักคลูชัน
material-role-rma = ความหยาบ โลหะ อักคลูชัน
material-role-height = ความสูง
material-role-bump = บัมป์
material-role-cavity = คาวิตี
material-role-specular = สเปคคิวลาร์
material-role-opacity = ความทึบ
material-role-emissive = การเปล่งแสง
material-role-translucency = ความโปร่งแสง
material-role-base-color-name = สี
material-role-normal-name = นอร์มอล
material-role-normal-dx-name = นอร์มอล (DirectX)
material-role-roughness-name = ความหยาบ
material-role-gloss-name = ความเงา
material-role-metalness-name = ความเป็นโลหะ
material-role-ao-name = แอมเบียนต์อักคลูชัน
material-role-orm-name = อักคลูชัน ความหยาบ โลหะ
material-role-mra-name = โลหะ ความหยาบ อักคลูชัน
material-role-rma-name = ความหยาบ โลหะ อักคลูชัน
material-role-height-name = ความสูง
material-role-bump-name = บัมป์
material-role-cavity-name = คาวิตี
material-role-specular-name = สเปคคิวลาร์
material-role-opacity-name = ความทึบ
material-role-emissive-name = การเปล่งแสง
material-role-translucency-name = ความโปร่งแสง
material-maps = { $count } แมป
material-dialog = ดูแมทีเรียล { $name }
material-up-to = สูงสุด { $size }
material-repeat-size = { $size } ต่อหนึ่งรอบการเรียงต่อ
material-shape-sphere = ทรงกลม
material-shape-cube = ลูกบาศก์
material-shape-plane = ระนาบ
material-shape-title = { $shape } ({ $key })
material-group-shape = รูปทรง
material-group-tiling = การเรียงต่อ
material-real-title = ขนาดจริง: หนึ่งรอบการเรียงต่อคือ { $size } ทรงกลมกว้าง 2 m (T เพื่อสลับ)
material-repeat-title = เรียงต่อ { $n }× (T เพื่อสลับ)
material-real = จริง
material-repeat = { $n }×
material-height-title = ระยะที่แมปความสูงดันพื้นผิวออกมา
material-height = ความสูง
material-height-strength = ความแรงของความสูง
material-loading = กำลังโหลดแมทีเรียล…
material-show-failed = แสดงแมทีเรียลนี้ไม่ได้: { $error }
material-preview-failed = แสดงตัวอย่างแมทีเรียลนี้ไม่ได้: { $error }
material-group-maps = แมป
material-map-show = แสดงแมป { $map }
material-map-hide = ซ่อนแมป { $map }
material-map-unused = ตัวอย่างไม่ได้ใช้
material-open-full-screen = เปิดตัวแสดงแมทีเรียลแบบเต็มจอ
material-keys-mouse = ลากเพื่อหมุนรอบ · ลากด้วยปุ่มขวาเพื่อเลื่อน · เลื่อนล้อเพื่อซูม · คลิกแมปเพื่อปิด
material-key-shape = รูปทรง
material-key-tiling = การเรียงต่อ
material-key-turn = หมุน

## theme.ts

theme-slate = หินชนวน
theme-paper = กระดาษ
theme-linen = ลินิน
theme-sage = เสจ
theme-lilac = ไลแลค
theme-blush = ชมพูระเรื่อ
theme-graphite = แกรไฟต์
theme-obsidian = ออบซิเดียน
theme-ember = ถ่านคุ
theme-moss = มอส
theme-aubergine = มะเขือม่วง
theme-oxblood = แดงเลือดนก
theme-sky = ท้องฟ้า
theme-mint = มินต์
theme-apricot = แอปริคอต
theme-lemon = เลมอน
theme-midnight = เที่ยงคืน
theme-lagoon = ลากูน
theme-neon = นีออน
theme-light-slate-blurb = เทาอมฟ้าโทนเย็น
theme-light-paper-blurb = เทากลางเรียบ ๆ
theme-light-linen-blurb = ครีมอุ่น อำพัน
theme-light-sage-blurb = เขียวอ่อนนุ่ม
theme-light-lilac-blurb = ม่วงอ่อนจาง
theme-light-blush-blurb = ชมพูกุหลาบหม่น
theme-light-sky-blurb = ฟ้าใส
theme-light-mint-blurb = มินต์สีฟ้าน้ำทะเลเย็นตา
theme-light-apricot-blurb = พีชอ่อน ส้ม
theme-light-lemon-blurb = เหลืองอ่อน สีดินเหลือง
theme-dark-graphite-blurb = เทากลาง ไม่มีโทนสี
theme-dark-slate-blurb = เทาอมฟ้า
theme-dark-obsidian-blurb = เกือบดำ สำหรับจอ OLED
theme-dark-ember-blurb = ถ่านโทนอุ่น อำพัน
theme-dark-moss-blurb = เขียวป่าเข้มเกือบดำ
theme-dark-aubergine-blurb = ม่วงพลัมเกือบดำ ไลแลค
theme-dark-oxblood-blurb = แดงไวน์เกือบดำ ปะการัง
theme-dark-midnight-blurb = กรมท่าเข้ม ฟ้า
theme-dark-lagoon-blurb = เขียวหัวเป็ดเข้ม ฟ้าน้ำทะเล
theme-dark-neon-blurb = เกือบดำ ชมพูสด

## texnames.ts

texnames-needs-map = รูปแบบต้องมี { "{" }map{ "}" } ไม่เช่นนั้นทุกแมปของแมทีเรียลจะได้ชื่อเดียวกัน
texnames-placeholders = รูปแบบใช้ { "{" }name{ "}" }, { "{" }map{ "}" } และ { "{" }size{ "}" } ได้
texnames-bad-characters = ชื่อต้องไม่มี < > : " / \ | ? หรือ *
texnames-no-suffix = กำหนดคำต่อท้ายให้แมป { $role }
texnames-bad-suffix = คำต่อท้ายของแมป { $role } ต้องไม่มี < > : " / \ | ? หรือ *
texnames-slot-no-suffix = กำหนดคำต่อท้ายให้ “{ $slot }”
texnames-slot-bad-suffix = คำต่อท้ายของ “{ $slot }” ต้องไม่มี < > : " / \ | ? หรือ *
texnames-ending-missing = ท้ายชื่อที่คุณกำหนดเองแต่ละรายการต้องระบุท้ายชื่อที่จะค้นหา
texnames-ending-bad = ท้ายชื่อที่คุณกำหนดเองต้องไม่มี < > : " / \ | ? หรือ *
texnames-ending-twice = { $ending } อยู่ในท้ายชื่อของคุณสองครั้ง
texnames-slot-metallic-roughness = โลหะและความหยาบ
texnames-slot-specular-color = สีสเปคคิวลาร์
texnames-slot-specular-glossiness = สเปคคิวลาร์และความเงา
texnames-slot-clearcoat = เคลียร์โค้ต
texnames-slot-clearcoat-roughness = ความหยาบของเคลียร์โค้ต
texnames-slot-clearcoat-normal = นอร์มอลของเคลียร์โค้ต
texnames-slot-sheen-color = สีชีน
texnames-slot-sheen-roughness = ความหยาบของชีน
texnames-slot-transmission = การส่งผ่านแสง
texnames-slot-thickness = ความหนา
texnames-slot-diffuse-transmission = การส่งผ่านแสงแบบกระจาย
texnames-slot-diffuse-transmission-color = สีการส่งผ่านแสงแบบกระจาย
texnames-slot-iridescence = ความเหลือบสี
texnames-slot-iridescence-thickness = ความหนาของชั้นเหลือบสี
texnames-slot-anisotropy = แอนไอโซทรอปี

## storeFormats.ts

storefiles-field-title = ชื่อเรื่อง
storefiles-field-title-hint = ชื่อที่ตรงกับในหน้าสินค้า ใช้ตั้งชื่อแพ็ก และหากไม่มีจะข้ามไฟล์นี้ไป
storefiles-field-seller = ผู้ขาย
storefiles-field-seller-hint = ผู้สร้างหรือผู้ขาย
storefiles-field-license = สัญญาอนุญาต
storefiles-field-license-hint = ตามที่ไฟล์เขียนไว้ เช่น CC-BY-4.0
storefiles-field-url = หน้าร้านค้า
storefiles-field-url-hint = ที่อยู่ของหน้าสินค้า ขึ้นต้นด้วย http:// หรือ https://
storefiles-field-words = แท็ก
storefiles-field-words-hint = คำที่จะใช้ติดแท็ก ค่าที่คั่นด้วยจุลภาคจะถูกแยก
storefiles-field-type = ประเภท
storefiles-field-type-hint = สิ่งนี้คืออะไร เช่น “3D model” หรือ “surface” ใช้เป็นแท็กด้วย
storefiles-field-colors = สี
storefiles-field-colors-hint = สีของสิ่งนี้ ในรูปแท็ก color:
storefiles-field-id = ID ร้านค้า
storefiles-field-id-hint = ID ที่ร้านค้าใช้: ตัวอักษร ตัวเลข _ หรือ -
storefiles-field-origin = สร้างที่
storefiles-field-origin-hint = สถานที่ที่สร้างหรือสแกน สำหรับคำอธิบาย
storefiles-field-size = ขนาด
storefiles-field-size-hint = ขนาดเป็นคำบรรยาย สำหรับคำอธิบาย
storefiles-field-title-name = ชื่อเรื่อง
storefiles-field-seller-name = ผู้ขาย
storefiles-field-license-name = สัญญาอนุญาต
storefiles-field-url-name = หน้าร้านค้า
storefiles-field-words-name = แท็ก
storefiles-field-type-name = ประเภท
storefiles-field-colors-name = สี
storefiles-field-id-name = ID ร้านค้า
storefiles-field-origin-name = สถานที่สร้าง
storefiles-field-size-name = ขนาด

## StoreFormatsPanel.vue

storefiles-load-failed = โหลดรูปแบบไฟล์ร้านค้าไม่ได้: { $error }
storefiles-remove-title = นำ { $name } ออกหรือไม่?
storefiles-remove-body = { $file } จะถูกลบออกจากโฟลเดอร์ข้อมูล แพ็กที่อ่านด้วยรูปแบบนี้แล้วจะคงสิ่งที่อ่านได้ไว้
storefiles-remove = นำออก
storefiles-removed = นำ { $name } ออกแล้ว
storefiles-remove-failed = นำ { $name } ออกไม่ได้: { $error }
storefiles-unread = ไฟล์ที่ Hoard ไม่ได้อ่าน
storefiles-loading = กำลังโหลด…
storefiles-unread-none = ไม่มี: ไม่มีไฟล์ในคลังที่ดูเหมือนไฟล์ของร้านค้าที่ยังไม่ได้อ่าน
storefiles-unread-what = ไฟล์ที่อยู่ในมากกว่าหนึ่งแพ็ก หรือตั้งชื่อแบบไฟล์ของร้านค้า ซึ่งไม่มีรูปแบบใดอ่าน
storefiles-unread-packs = { $count } แพ็ก
storefiles-unread-example = เช่น { $pack }
storefiles-teach = สอนให้ Hoard รู้จักไฟล์นี้
storefiles-formats = รูปแบบ
storefiles-formats-what = รูปแบบของคุณจะถูกลองก่อน และใช้แทนรูปแบบในตัวที่มีชื่อเดียวกัน
storefiles-dir-before = เก็บไว้ใน
storefiles-dir-after = { "" }
storefiles-yours = ของคุณ
storefiles-built-in = ในตัว
storefiles-edit = แก้ไข
storefiles-start-from = เริ่มจากรูปแบบนี้

## TagsPanel.vue

tags-load-failed = โหลดแท็กไม่ได้: { $error }
tags-rename-title = เปลี่ยนชื่อ { $tag } หรือไม่?
tags-merge-title = รวม { $count } แท็กหรือไม่?
tags-rename-body = แอสเซ็ต { $count } รายการจะได้แท็ก { $into } แทน และตัวติดแท็กจะใช้ชื่อใหม่นับจากนี้
tags-merge-body = { $tags } จะกลายเป็น { $into } ในแอสเซ็ตสูงสุด { $count } รายการ ตัวติดแท็กจะใช้ { $into } นับจากนี้
tags-rename = เปลี่ยนชื่อ
tags-merge = รวม
tags-merged = ตอนนี้แอสเซ็ต { $count } รายการติดแท็ก { $into } แล้ว
tags-merge-failed = รวมไม่ได้: { $error }
tags-forget-failed = เปลี่ยนไม่ได้: { $error }
tags-find = ค้นหาแท็ก
tags-find-placeholder = ค้นหาแท็ก เช่น rock หรือ set:
tags-sort = ลำดับ
tags-sort-count = ใช้มากที่สุด
tags-sort-name = ก–ฮ
tags-count = { $count } แท็ก
tags-count-more = { $count }+ แท็ก
tags-what = เลือกแท็กที่มีความหมายเดียวกันแล้วรวมเข้าด้วยกัน หรือเลือกแท็กเดียวเพื่อเปลี่ยนชื่อ
tags-new-name = ชื่อใหม่
tags-merge-into = รวมเข้ากับ
tags-saved-as-before = บันทึกเป็น
tags-saved-as-after = { "" }
tags-cancel = ยกเลิก
tags-loading = กำลังโหลด…
tags-none = ไม่มีแท็ก
tags-none-containing = ไม่มีแท็กที่มี “{ $text }”
tags-browse-title = เรียกดูแอสเซ็ตที่ติดแท็ก { $tag }
tags-browse = เรียกดู
tags-merged-heading = แท็กที่รวมแล้ว
tags-merged-what = เมื่อตัวติดแท็กเสนอชื่อเก่าเหล่านี้ แอสเซ็ตจะได้แท็กที่ชื่อนั้นถูกรวมเข้าไปแทน
tags-allow-old = อนุญาตชื่อเก่าอีกครั้ง
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count = { $count } แอสเซ็ต
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
       *[other] ชื่อเดิม: { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } ถูกรวมเข้ากับ { $into } แล้ว
tag-page-show = แสดง { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = บอกว่าแท็กนี้หมายถึงอะไร…
tag-page-describe-label = แท็กนี้หมายถึงอะไร
tag-page-describe-placeholder = ใช้ทำอะไร และอะไรอยู่ภายใต้แท็กนี้ กด Enter เพื่อบันทึก
tag-page-describe-edit = คลิกเพื่อเปลี่ยน
tag-page-description-saved = บันทึกความหมายของแท็กแล้ว
# The tag's picture: one of its assets stands for it.
tag-page-picture = เลือกรูปภาพสำหรับแท็กนี้
tag-page-picture-help = เลือกแอสเซ็ตหนึ่งรายการเพื่อเป็นตัวแทนของแท็ก
tag-page-picture-nothing = แอสเซ็ตที่แสดงอยู่ไม่มีรูปภาพเลย
tag-page-picture-none = ไม่มีรูปภาพ
tag-page-picture-saved = ตั้งรูปภาพแล้ว
tag-page-picture-removed = เอารูปภาพออกแล้ว
tag-page-rename = เปลี่ยนชื่อ…
tag-page-rename-title = ตั้งชื่อใหม่ หรือใส่ชื่อแท็กอื่นเพื่อรวมเข้าด้วยกัน
tag-page-rename-label = ชื่อใหม่
tag-page-save-failed = บันทึกแท็กไม่ได้: { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = อยู่ใต้
tag-page-parent-add = ใส่ไว้ใต้…
tag-page-parent-add-title = ใส่แท็กนี้ไว้ใต้แท็กที่กว้างกว่า: ค้นหาแท็กนั้นแล้วจะเจอแท็กเหล่านี้ด้วย
tag-page-parent-placeholder = แท็กที่กว้างกว่า เช่น ยานพาหนะ
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = นำ { $tag } ออกจากใต้ { $parent }
tag-page-parent-saved = { $tag } อยู่ใต้ { $parent } แล้ว
tag-page-parent-removed = { $tag } ไม่อยู่ใต้ { $parent } แล้ว
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = แคบกว่า:
tag-page-narrower-title = ค้นหา { $tag } จะเจอแท็กเหล่านี้ด้วย
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = กว้างกว่าและแคบกว่า
tags-tree-what = ใส่แท็กไว้ใต้แท็กที่กว้างกว่า ที่นี่หรือในหน้าของแท็กนั้น แล้วการค้นหาแท็กที่กว้างกว่าจะเจอทุกอย่างที่อยู่ใต้มันด้วย
tags-tree-none = ยังไม่มีแท็กใดอยู่ใต้แท็กอื่น ลองใส่ “รถยนต์” ไว้ใต้ “ยานพาหนะ” ดู
tags-tree-tag-label = แท็กที่จะใส่ไว้ใต้แท็กที่กว้างกว่า
tags-tree-tag-placeholder = แท็ก เช่น รถยนต์
tags-tree-goes-under = อยู่ใต้
tags-tree-parent-label = แท็กที่กว้างกว่า
tags-tree-put = ใส่ไว้ใต้

## LibraryStats.vue

stats-source-human = เพิ่มโดยผู้ใช้
stats-source-ai = แนะนำโดย AI
stats-source-rule = กฎในตัว
stats-source-meta = จากไฟล์
stats-assets = แอสเซ็ต
stats-packs = แพ็ก
stats-on-disk = บนดิสก์
stats-duplicates-title = มี { $count } ไฟล์ที่เป็นสำเนาเหมือนไฟล์อื่นทุกไบต์ และจัดเก็บเพียงครั้งเดียว
stats-saved-dedup = ประหยัดได้จากการตัดข้อมูลซ้ำ
stats-distinct-tags = แท็กที่แตกต่างกัน
stats-archive = ในที่เก็บถาวร
stats-load-failed = โหลดรายละเอียดไม่ได้: { $error }
stats-counting = กำลังนับ…
stats-as-of = ณ { $time }
stats-updating = กำลังอัปเดต…
stats-by-kind = พื้นที่ตามประเภท
stats-files-size = { $name }: { $count } ไฟล์, { $size }
stats-months = เพิ่มในหนึ่งปีที่ผ่านมา
stats-month = { $month }: { $count } แอสเซ็ต, { $size }
stats-tagging = การติดแท็ก
stats-tagged = ติดแท็กแล้ว
stats-untagged = ไม่มีแท็ก { $count } รายการ
stats-described = มีคำอธิบาย
stats-described-assets = { $count } แอสเซ็ต
stats-human-edited = แก้ไขด้วยมือ
stats-human-edited-unit =
    { $count ->
       *[other] แอสเซ็ต
    }
stats-source-different = { $count } แบบ
stats-most-used = ใช้มากที่สุด
stats-show-tag = แสดงแอสเซ็ต { $tag }
stats-file-types = ประเภทไฟล์
stats-biggest-packs = แพ็กที่ใหญ่ที่สุด
stats-browse-pack = เรียกดู { $name }
stats-largest-files = ไฟล์ที่ใหญ่ที่สุด
stats-open-file = เปิด { $name }
stats-empty = ยังไม่มีอะไรในคลัง เพิ่มแพ็กแล้วตัวเลขจะแสดงที่นี่

## HoardPanel.vue

hoard-title = ขุมทรัพย์ของคุณ
hoard-what = แอสเซ็ตทุกชิ้นในคลังคือสมบัติหนึ่งชิ้น และชนิดของแอสเซ็ตกำหนดว่าเป็นสมบัติแบบใด เก็บของใหม่เพื่อเพิ่มลงในกองของคุณ
hoard-sign-in = ลงชื่อเข้าใช้เพื่อสะสมขุมทรัพย์ของคุณเอง
hoard-new = ของรางวัลใหม่ที่เก็บได้: { $count }
hoard-none-new = ไม่มีของใหม่ให้เก็บ
hoard-collect = เก็บ
hoard-unavailable = เบราว์เซอร์นี้วาดขุมทรัพย์ไม่ได้: { $error }
hoard-collect-failed = บันทึกสิ่งที่คุณเก็บไม่ได้: { $error }
hoard-arcade = จออาร์เคด
hoard-full-screen = เต็มจอ
hoard-leave-full-screen = ออกจากโหมดเต็มจอ
hoard-label = มังกรเฝ้าขุมทรัพย์ของมัน สมบัติ: { $count }
hoard-gold = ทองคำ
hoard-gems = อัญมณี
hoard-books = หนังสือ
hoard-weapons = อาวุธ
hoard-bones = กระดูก
hoard-tablets = แผ่นจารึกรูน
hoard-paintings = ภาพวาด
hoard-potions = ยา

## UpdatesPanel.vue

updates-saved = Hoard จะเรียกที่อยู่นี้เมื่อมีอัปเดตพร้อม
updates-saved-none = ไม่มีที่อยู่ที่จะเรียก
updates-save-failed = บันทึกไม่ได้
updates-call-answered = ตอบกลับด้วย { $result }
updates-call-refused = ไม่รับคำขอ: { $result }
updates-call-failed = เรียกไม่ได้
updates-called = เรียกเมื่อ { $when } สำหรับ { $version }: { $result }
updates-called-failed = เรียกเมื่อ { $when } สำหรับ { $version }: { $result } Hoard จะลองอีกครั้งในอีกหนึ่งชั่วโมง
updates-latest = Hoard { $version } เป็นรุ่นล่าสุด
updates-out = Hoard { $version } ออกแล้ว
updates-out-idle = Hoard { $version } ออกแล้ว จะเรียกภายในสองสามนาทีขณะที่ไม่มีงานใดทำอยู่
updates-out-busy = Hoard { $version } ออกแล้ว กำลังรอจนกว่าจะไม่มีงานใดทำอยู่ ({ $work })
updates-busy-more = อีก { $count } รายการ
updates-busy-jobs = งานรออยู่ { $count } งาน
updates-busy-uploads = อัปโหลด { $count } รายการ
updates-list-separator = ,{ " " }
updates-what = เมื่อมีรุ่นใหม่ออก Hoard สามารถเรียกที่อยู่หนึ่งเมื่อไม่มีงานใดทำอยู่ (ไม่มีการอัปโหลด งาน การแปลง หรือการบำรุงรักษา) เพื่อให้สิ่งที่อัปเดตคอนเทนเนอร์ไม่รีสตาร์ต Hoard กลางการอัปโหลด โดยจะส่ง POST พร้อมรุ่นในเนื้อหา JSON ซึ่งต้องเปิดการตรวจหารุ่นใหม่ด้านบนไว้
updates-url = ที่อยู่ที่จะเรียก
updates-url-placeholder = ไม่มี (เช่น http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = บันทึกแล้ว (พิมพ์เพื่อแทนที่)
updates-auth-placeholder = ไม่มี (เช่น Bearer my-token)
updates-remove = นำออก
updates-save = บันทึก
updates-call-now-title = สิ่งที่เชื่อมต่อไว้จะทำงาน
updates-call-now = เรียกตอนนี้
updates-what-calls = เรียกอะไรได้บ้าง
updates-watchtower = HTTP API ของ Watchtower
updates-watchtower-at = :{ " " }
updates-watchtower-with = { " " }พร้อม{ " " }
updates-watchtower-token = { " " }และโทเค็นจาก{ " " }
updates-watchtower-its = { " " }(เปิดปลายทาง{ " " }
updates-watchtower-endpoint = { " " }ไว้)
updates-portainer = Webhook ของ Portainer
updates-portainer-rest = { " " }บนสแต็กหรือคอนเทนเนอร์ของ Hoard โดยเปิดการดึงอิมเมจใหม่ไว้ โทเค็นอยู่ใน URL
updates-script = สคริปต์บนโฮสต์
updates-script-listener = { " " }ที่อยู่หลังตัวรับ webhook (เช่น{ " " }
updates-script-runs = ) ซึ่งเรียกใช้{ " " }
updates-script-end = { "" }
updates-others = Home Assistant, n8n
updates-others-rest = { " " }หรือสิ่งอื่นใดที่รับ webhook ได้
updates-poll-before = หรือไม่ต้องใช้ที่อยู่ก็ได้: สคริปต์ที่มีโทเค็นการเข้าถึงสามารถเรียก{ " " }
updates-poll-until = { " " }เป็นระยะจนกว่า{ " " }
updates-poll-and = { " " }จะถูกตั้งค่าและ{ " " }
updates-poll-after = { " " }เป็น true แล้วจึงดึงอิมเมจและรีสตาร์ต Hoard

## MaintenancePanel.vue

maintenance-load-failed = โหลดการบำรุงรักษาไม่ได้
maintenance-reanalyze-title = วิเคราะห์ทุกแพ็กใหม่หรือไม่?
maintenance-reanalyze-body = ทุกไฟล์จะถูกอ่านอีกครั้ง ซึ่งอาจใช้เวลานานสำหรับคลังขนาดใหญ่ แพ็กจะแสดงว่ากำลังดำเนินการจนกว่าจะถึงคิว ชื่อ แท็ก และโน้ตที่คุณตั้งเองจะยังคงอยู่
maintenance-reanalyze-confirm = วิเคราะห์ใหม่
maintenance-gc-title = ลบไฟล์ที่ไม่ได้ใช้หรือไม่?
maintenance-gc-body = ไฟล์ที่จัดเก็บไว้ซึ่งไม่มีสิ่งใดในคลังอ้างถึงจะถูกลบถาวร แพ็กในที่เก็บถาวรยังคงเก็บไฟล์ของตนไว้
maintenance-gc-confirm = ลบไฟล์เหล่านั้น
maintenance-start-failed = เริ่ม “{ $task }” ไม่ได้
maintenance-just-now = เมื่อสักครู่
maintenance-minutes-ago = { $count } นาทีที่แล้ว
maintenance-hours-ago = { $count } ชม. ที่แล้ว
maintenance-took-seconds = { $count } วินาที
maintenance-took-minutes = { $count } นาที
maintenance-took-hours = { $hours } ชม.
maintenance-loading = กำลังโหลด…
maintenance-running = กำลังทำงาน…
maintenance-starting = กำลังเริ่ม…
maintenance-run = เรียกใช้
maintenance-stop = หยุด
maintenance-stopping = กำลังหยุด…
maintenance-stop-failed = หยุด “{ $task }” ไม่ได้
maintenance-started = { $detail } · เริ่มเมื่อ { $when }
maintenance-starting-detail = กำลังเริ่ม
maintenance-finished = เสร็จสิ้น:{ " " }
maintenance-failed = ล้มเหลว:{ " " }
maintenance-never-run = ยังไม่เคยเรียกใช้
maintenance-history = การทำงานที่ผ่านมา
maintenance-col-task = งาน
maintenance-col-finished = เสร็จเมื่อ
maintenance-col-took = ใช้เวลา
maintenance-col-result = ผลลัพธ์
maintenance-script-before = การสำรองข้อมูลและการตรวจหาความเสียหายเรียกใช้จากสคริปต์ได้ด้วย:{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }และ{ " " }
maintenance-script-after = { " " }การทำงานเหล่านั้นจะแสดงที่นี่ด้วย

## SettingsPage.vue

settings-title = การตั้งค่า
settings-loading = กำลังโหลด…
settings-find = ค้นหาการตั้งค่า
settings-find-none = ไม่มีการตั้งค่าที่ตรงกับ “{ $query }”
settings-load-failed = โหลดการตั้งค่าไม่ได้: { $error }
settings-page-appearance = รูปลักษณ์
settings-page-appearance-blurb = ภาษา ธีม ขนาดข้อความและระยะห่าง และสิ่งที่ไทล์และแถบด้านบนแสดง บันทึกไว้ในเบราว์เซอร์นี้เท่านั้น
settings-page-dragon = มังกร
settings-page-dragon-blurb = ขุมสมบัติของมังกรคุณ: ไฟล์ทุกชิ้นในคลังคือสมบัติชิ้นหนึ่ง
settings-page-library = คลัง
settings-page-library-blurb = สิ่งที่อยู่ในคลัง: ขนาดเท่าใด ประกอบด้วยอะไร เติบโตมาอย่างไร และติดแท็กไว้ดีเพียงใด ไม่นับสิ่งที่เก็บถาวรไว้
settings-page-copies = สำเนาที่เกือบเหมือนกัน
settings-page-copies-blurb = ไฟล์ที่เป็นภาพ เสียง หรือโมเดลเดียวกันในขนาด อัตราสุ่มตัวอย่าง หรือรูปแบบอื่น เรียงจากที่ใช้พื้นที่มากที่สุดก่อน สำเนาที่ดีที่สุดของแต่ละชุดมีเครื่องหมายไว้ ไม่มีการลบสิ่งใด
copies-loading = กำลังค้นหาสำเนาที่เกือบเหมือนกัน…
copies-refreshing = คลังมีการเปลี่ยนแปลงตั้งแต่พบรายการเหล่านี้ กำลังค้นหาอีกครั้ง…
copies-none = ไม่มีสำเนาที่เกือบเหมือนกัน: ภาพ เสียง และโมเดลทุกชิ้นมีอยู่ที่นี่เพียงครั้งเดียว
copies-load-failed = แสดงรายการสำเนาที่เกือบเหมือนกันไม่ได้: { $error }
copies-sum = ชุดสำเนาที่เกือบเหมือนกัน: { $count } สำเนาที่ไม่ใช่ตัวดีที่สุดใช้พื้นที่ { $size }
copies-set = ชุดที่ { $n }
copies-set-wasted = { $size } ในสำเนาที่ไม่ใช่ตัวดีที่สุด
copies-more = แสดงเพิ่ม (เหลืออีก { $count })
settings-page-tags = แท็ก
settings-page-tags-blurb = แท็กทั้งหมดในคลัง รวมแท็กที่มีความหมายเดียวกัน หรือเปลี่ยนชื่อแท็ก ตัวติดแท็กจะใช้ชื่อใหม่นับจากนั้น
settings-page-store-files = ไฟล์ร้านค้า
settings-page-store-files-blurb = ไฟล์ที่ร้านค้าใส่ไว้ข้างงานศิลป์ ซึ่ง Hoard อ่านเพื่อตั้งชื่อแพ็กและให้เครดิตผู้สร้าง สอนไฟล์ที่ Hoard ยังไม่รู้จัก และแบ่งปันสิ่งที่คุณสอน
settings-page-locations = ตำแหน่งคลัง
settings-page-locations-blurb = โฟลเดอร์บนเซิร์ฟเวอร์ที่เก็บคลัง: โฟลเดอร์ที่ Hoard เก็บไฟล์ไว้ และโฟลเดอร์ของแพ็กที่ Hoard จัดทำดัชนีไว้ในที่เดิม
settings-page-locations-blurb-desktop = โฟลเดอร์ที่เก็บคลัง: โฟลเดอร์ที่ Hoard เก็บไฟล์ไว้ และโฟลเดอร์ของแพ็กที่ Hoard จัดทำดัชนีไว้ในที่เดิม
settings-page-tools = เครื่องมือ
settings-page-tools-blurb = โปรแกรมภายนอกที่ Hoard ใช้แปลงและอ่านไฟล์
settings-page-ai = การติดแท็กด้วย AI
settings-page-ai-blurb = โมเดล AI ตั้งชื่อ อธิบาย และติดแท็กแพ็กใหม่ เพิ่มเติมจากกฎในตัว แท็กของโมเดลจะถูกทำเครื่องหมายว่าเป็นคำแนะนำ
settings-page-search-blurb = ค้นหารูปภาพจากสิ่งที่ปรากฏในภาพ และค้นหาเสียงจากสิ่งที่ได้ยิน ไม่ใช่แค่จากชื่อ และค้นหาสิ่งที่เป็นประเภทเดียวกัน ไม่ใช่แค่สีคล้ายกัน
settings-page-search-tab = การค้นหา
settings-page-adding = การเพิ่มแพ็ก
settings-page-adding-blurb = สิ่งที่ Hoard ทำกับแพ็กเมื่อถูกเพิ่ม นอกเหนือจากการอ่านและติดแท็ก
settings-page-downloads = การดาวน์โหลด
settings-page-downloads-blurb = วิธีตั้งชื่อไฟล์ในสิ่งที่ Hoard ส่งออกไป: การดาวน์โหลด ไฟล์ zip เครื่องมือ hoard และผู้ช่วย AI ไฟล์ในคลังยังคงเหมือนเดิม
settings-page-account = บัญชี
settings-page-account-blurb = รหัสผ่านของคุณ คนอื่นที่ลงชื่อเข้าใช้ได้ และโทเค็นการเข้าถึงสำหรับสคริปต์และโปรแกรมอื่น
settings-page-auth = การยืนยันตัวตน
settings-page-auth-blurb = วิธีที่ผู้คนลงชื่อเข้าใช้: บัญชีของ Hoard เอง ไดเรกทอรี LDAP, Microsoft Active Directory หรือผู้ให้บริการ OAuth เซสชันอยู่ได้นานเท่าใด และกลุ่มใดได้บทบาทใด
settings-page-connection = การเชื่อมต่อ
settings-page-connection-blurb = วิธีที่เบราว์เซอร์และแอปเข้าถึง Hoard: ที่อยู่ที่ใช้เรียก Hoard และ HTTPS ซึ่งฟีเจอร์บางอย่างของเบราว์เซอร์ต้องใช้
settings-page-performance = ประสิทธิภาพ
settings-page-performance-blurb = เซิร์ฟเวอร์ทำงานหนักเพียงใด ภาพย่อถูกสร้างและเก็บอย่างไร และรับข้อมูลได้มากที่สุดเท่าใด
settings-page-performance-blurb-desktop = Hoard ทำงานหนักเพียงใด ภาพย่อถูกสร้างและเก็บอย่างไร และรับข้อมูลได้มากที่สุดเท่าใด
settings-page-issues = ปัญหา
settings-page-issues-blurb = สิ่งที่ผิดพลาดขณะเปิดหรือประมวลผลไฟล์: ไฟล์บีบอัดที่แตกไม่ได้ ไฟล์ที่อ่านไม่ได้ งานที่ล้มเหลว ลองใหม่เมื่อแก้สาเหตุแล้ว หรือปิดรายการเหล่านั้น
settings-page-maintenance = การบำรุงรักษา
settings-page-maintenance-blurb = สำรองข้อมูล ตรวจหาความเสียหาย และจัดระเบียบ แต่ละอย่างทำงานได้ทีละหนึ่งรายการ และคุณออกจากหน้านี้ได้ขณะที่ทำงานอยู่
settings-page-about = เกี่ยวกับ
settings-page-about-blurb = Hoard คืออะไร รุ่นนี้คือรุ่นใด และผลงานของผู้อื่นที่ใช้สร้าง Hoard
settings-the-default = ค่าเริ่มต้น
settings-default-on = เปิด
settings-default-off = ปิด
settings-default-auto = อัตโนมัติ
settings-option-saved = { $name }: { $value }
settings-back-to-default = กลับเป็นค่าเริ่มต้น
settings-save = บันทึก
settings-use = ใช้
settings-status-off = ปิด

## SettingsPage.vue: Appearance

settings-themes = ธีม
settings-look-light = สว่าง
settings-look-dark = มืด
settings-look-group =
    { $scheme ->
        [light] ธีมสว่าง
       *[dark] ธีมมืด
    }
settings-preview =
    กำลังแสดงธีม{ $preview ->
        [light] สว่าง
       *[dark] มืด
    }ให้คุณดู คุณกำลังใช้โหมด{ $used ->
        [light] สว่าง
       *[dark] มืด
    }{ $system ->
        [yes] { " " }(ตามระบบ)
       *[other] { "" }
    } และจะกลับมาเป็นแบบนั้นเมื่อคุณออกจากหน้านี้
settings-preview-back =
    กลับไปที่โหมด{ $used ->
        [light] สว่าง
       *[dark] มืด
    }
settings-look-hint = สลับระหว่างสว่างและมืด (หรือตามระบบ) ได้จากแถบด้านบน

## SettingsPage.vue: Tools

settings-blender-what = แปลงโมเดล .blend, .dae, .abc และ .x3d เป็น glTF ในเบื้องหลัง เพื่อให้มีภาพย่อและตัวอย่าง 3 มิติ แพ็กที่เพิ่มขณะที่ปิดอยู่จะถูกแปลงเมื่อพร้อม
settings-blender-download = ใช้ Blender ที่ติดตั้งไว้ หรือดาวน์โหลด Blender 4.5 LTS (ประมาณ 400 MB) จากมิเรอร์ของ blender.org
settings-ffmpeg-what = เข้ารหัสไฟล์ดาวน์โหลด MP3, OGG และ Opus และอ่านเสียงที่ Hoard ถอดรหัสเองไม่ได้ (AAC, M4A, WMA, …)
settings-ffmpeg-download = ใช้ ffmpeg ของระบบที่มีตัวเข้ารหัสที่ถูกต้อง หรือดาวน์โหลดบิลด์แบบสแตติก (ประมาณ 80 MB)
settings-tool-reset = { $tool }: กลับเป็นค่าเริ่มต้น
settings-tool-no-download = ใช้ { $tool } ที่ติดตั้งไว้ ไม่มีบิลด์ให้ดาวน์โหลดสำหรับเครื่องนี้ จึงต้องติดตั้งเองหรือระบุพาธ
settings-tool-path = พาธโปรแกรม { $tool }
settings-mode-off = ปิด
settings-mode-auto = อัตโนมัติ
settings-mode-path = โปรแกรมนี้
settings-tool-looking = กำลังค้นหา…
settings-tool-downloading = กำลังดาวน์โหลด… ดูความคืบหน้าได้ที่ตัวแสดงกิจกรรม
settings-tool-ready = พร้อม: { $detail }
settings-soundfont = SoundFont สำหรับ MIDI
settings-soundfont-what = เครื่องดนตรีที่ใช้เล่นและแปลงไฟล์ MIDI
settings-soundfont-file = ไฟล์ SoundFont
settings-soundfont-placeholder = ที่พบบนเซิร์ฟเวอร์ หรือแบบในตัว
settings-soundfont-placeholder-desktop = ที่พบบนคอมพิวเตอร์เครื่องนี้ หรือแบบในตัว
settings-soundfont-default = ที่พบบนเซิร์ฟเวอร์ หรือแบบในตัว
settings-soundfont-default-desktop = ที่พบบนคอมพิวเตอร์เครื่องนี้ หรือแบบในตัว
settings-soundfont-send = เลือกไฟล์…
settings-soundfont-sent = ตอนนี้ MIDI เล่นด้วย { $name }

## SettingsPage.vue: Image search

settings-clip-heading = ค้นหาจากสิ่งที่ปรากฏในภาพ
settings-clip-reset = ค้นหาด้วยภาพ: กลับเป็นค่าเริ่มต้น
settings-clip-saved-on = ค้นหาด้วยภาพ: เปิด
settings-clip-saved-off = ค้นหาด้วยภาพ: ปิด
settings-clip-what = พิมพ์สิ่งที่คุณต้องการ (“หินมีมอส”, “ประตูไซไฟ”, “ถังไม้”) แล้วรูปภาพที่แสดงสิ่งนั้นจะปรากฏข้างรายการที่มีคำเหล่านั้นในชื่อ “คล้ายกับรายการนี้” จะค้นหาสิ่งที่เป็นประเภทเดียวกัน ไม่ใช่แค่สีเดียวกัน
settings-clip-toggle = เปิดการค้นหาด้วยภาพ
settings-clip-hint = ใช้โมเดล SigLIP 2 ของ Google ที่ทำงานบนโปรเซสเซอร์ของเซิร์ฟเวอร์นี้: ไม่มีข้อมูลใดออกไปภายนอก การเปิดจะดาวน์โหลดโมเดล ({ $size } MB) จาก Hugging Face แล้วเรียนรู้รูปภาพทุกภาพในเบื้องหลังด้วยลำดับความสำคัญต่ำ ใช้หน่วยความจำ 350 ถึง 750 MB ขณะทำงานหรือขณะมีคนค้นหา การปิดจะลบโมเดลแต่เก็บสิ่งที่เรียนรู้ไว้ใช้ครั้งหน้า
settings-clip-hint-desktop = ใช้โมเดล SigLIP 2 ของ Google ที่ทำงานบนโปรเซสเซอร์ของคอมพิวเตอร์เครื่องนี้: ไม่มีข้อมูลใดออกไปภายนอก การเปิดจะดาวน์โหลดโมเดล ({ $size } MB) จาก Hugging Face แล้วเรียนรู้รูปภาพทุกภาพในเบื้องหลังด้วยลำดับความสำคัญต่ำ ใช้หน่วยความจำ 350 ถึง 750 MB ขณะทำงานหรือขณะที่คุณค้นหา การปิดจะลบโมเดลแต่เก็บสิ่งที่เรียนรู้ไว้ใช้ครั้งหน้า
settings-clip-looking = กำลังค้นหาโมเดล…
settings-clip-downloading = กำลังดาวน์โหลดโมเดล… ดูความคืบหน้าได้ที่ตัวแสดงกิจกรรม
settings-clip-ready-empty = พร้อม รูปภาพจะถูกเรียนรู้เมื่อมีการเพิ่มแพ็ก
settings-clip-ready-all =
    { $count ->
       *[other] พร้อม: รู้จักรูปภาพครบทั้ง { $count } ภาพ
    }
settings-clip-learning = พร้อม: กำลังเรียนรู้รูปภาพ { $done } จาก { $total } ภาพแล้ว
settings-clip-relearning = จนกว่าโมเดลใหม่จะเรียนรู้ภาพทุกภาพ การค้นหาจะใช้สิ่งที่โมเดลเก่าเรียนรู้ไว้
settings-clip-unavailable = รับโมเดลไม่ได้: { $detail } จะลองใหม่ทุกชั่วโมง
settings-clip-where = ทำงานบน
settings-clip-here = เซิร์ฟเวอร์นี้: ไม่มีอะไรออกไปข้างนอก
settings-clip-here-desktop = คอมพิวเตอร์เครื่องนี้: ไม่มีอะไรออกไปข้างนอก
settings-clip-hosted = บริการแบบโฮสต์
settings-clip-hosted-warning = ภาพจะออกจากเซิร์ฟเวอร์นี้ ภาพย่อของทุกภาพในคลัง และคำในการค้นหาด้วยภาพทุกครั้ง จะถูกส่งไปยัง { $service } ซึ่งคิดค่าบริการและเก็บไว้ตามข้อกำหนดของบริการนั้น
settings-clip-hosted-warning-desktop = ภาพจะออกจากคอมพิวเตอร์เครื่องนี้ ภาพย่อของทุกภาพในคลัง และคำในการค้นหาด้วยภาพทุกครั้ง จะถูกส่งไปยัง { $service } ซึ่งคิดค่าบริการและเก็บไว้ตามข้อกำหนดของบริการนั้น
settings-clip-hosted-hint = ไม่มีการดาวน์โหลดหรือเรียกใช้อะไรที่นี่ บริการจะเรียนรู้ทุกภาพทีละชุด บริการหรือโมเดลอื่นจะเรียนรู้ใหม่ทั้งหมด และสิ่งที่แต่ละตัวเรียนรู้ไว้จะถูกเก็บไว้เผื่อเปลี่ยนกลับ
settings-clip-saved-here = ค้นหาด้วยภาพ: ทำงานบนเซิร์ฟเวอร์นี้
settings-clip-saved-here-desktop = ค้นหาด้วยภาพ: ทำงานบนคอมพิวเตอร์เครื่องนี้
settings-clip-saved-hosted = ค้นหาด้วยภาพ: ทำงานบน { $service }
settings-clip-hosted-unavailable = ใช้ { $service } ไม่ได้: { $detail } จะลองใหม่ทุกชั่วโมง
settings-clap-heading = ค้นหาจากลักษณะของเสียงที่ได้ยิน
settings-clap-reset = ค้นหาด้วยเสียง: กลับเป็นค่าเริ่มต้น
settings-clap-saved-on = ค้นหาด้วยเสียง: เปิด
settings-clap-saved-off = ค้นหาด้วยเสียง: ปิด
settings-clap-what = พิมพ์สิ่งที่คุณต้องการ (“ฟ้าร้องไกลๆ”, “เสียงบลิป UI ใสๆ”, “เสียงฝีเท้าบนกรวด”) แล้วเสียงที่ฟังดูแบบนั้นจะปรากฏข้างรายการที่มีคำเหล่านั้นในชื่อ
settings-clap-toggle = เปิดการค้นหาด้วยเสียง
settings-clap-hint = ใช้โมเดล CLAP ของ LAION ที่ทำงานบนโปรเซสเซอร์ของเซิร์ฟเวอร์นี้: ไม่มีข้อมูลใดออกไปภายนอก การเปิดจะดาวน์โหลดโมเดล ({ $size } MB) จาก Hugging Face แล้วฟังช่วงต้นของเสียงทุกเสียงในเบื้องหลังด้วยลำดับความสำคัญต่ำ ใช้หน่วยความจำสูงสุดประมาณ 1 GB ขณะทำงานหรือขณะมีคนค้นหา การปิดจะลบโมเดลแต่เก็บสิ่งที่เรียนรู้ไว้ใช้ครั้งหน้า
settings-clap-hint-desktop = ใช้โมเดล CLAP ของ LAION ที่ทำงานบนโปรเซสเซอร์ของคอมพิวเตอร์เครื่องนี้: ไม่มีข้อมูลใดออกไปภายนอก การเปิดจะดาวน์โหลดโมเดล ({ $size } MB) จาก Hugging Face แล้วฟังช่วงต้นของเสียงทุกเสียงในเบื้องหลังด้วยลำดับความสำคัญต่ำ ใช้หน่วยความจำสูงสุดประมาณ 1 GB ขณะทำงานหรือขณะที่คุณค้นหา การปิดจะลบโมเดลแต่เก็บสิ่งที่เรียนรู้ไว้ใช้ครั้งหน้า
settings-clap-ready-empty = พร้อม เสียงจะถูกเรียนรู้เมื่อมีการเพิ่มแพ็ก
settings-clap-ready-all =
    { $count ->
       *[other] พร้อม: รู้จักเสียงครบทั้ง { $count } เสียง
    }
settings-clap-learning = พร้อม: กำลังเรียนรู้เสียง { $done } จาก { $total } เสียงแล้ว

## SettingsPage.vue: AI tagging

settings-ai-service = บริการ
settings-ai-off = ปิด
settings-ai-off-hint = แพ็กจะถูกติดแท็กด้วยกฎในตัวเท่านั้น
settings-ai-local = โมเดลในเครื่อง
settings-ai-local-hint = บริการ llm ในไฟล์ compose ของ Hoard (หรือ cargo xtask llm) ดาวน์โหลดโมเดลด้านล่าง โมเดลจะทำงานบนเครื่องนี้และไม่มีข้อมูลใดออกไปภายนอก
settings-ai-chatgpt = แพ็กเกจ ChatGPT
settings-ai-chatgpt-hint = ใช้แพ็กเกจ ChatGPT Go, Plus, Pro หรือ Business ของคุณแทนคีย์ API
settings-ai-openrouter-hint = คีย์เดียวสำหรับโมเดลที่โฮสต์ไว้หลายร้อยรายการ บางรายการใช้ฟรี โมเดลที่รับรูปภาพได้จะมีเครื่องหมายกำกับ
settings-ai-openai-hint = โฮสต์โดย OpenAI
settings-ai-anthropic-hint = โฮสต์โดย Anthropic ผ่านปลายทางที่เข้ากันได้กับ OpenAI
settings-ai-gemini-hint = โฮสต์โดย Google ผ่านปลายทางที่เข้ากันได้กับ OpenAI
settings-ai-ollama-hint = เซิร์ฟเวอร์ Ollama ที่คุณเรียกใช้เอง เปลี่ยนที่อยู่หากไม่ใช่คอนเทนเนอร์ชื่อ ollama
settings-ai-custom = อื่น ๆ (เข้ากันได้กับ OpenAI)
settings-ai-custom-hint = สิ่งใดก็ได้ที่ใช้ API แชตของ OpenAI: LM Studio, vLLM, llama.cpp, LiteLLM, …
settings-ai-connecting = กำลังเชื่อมต่อ…
settings-ai-connected = เชื่อมต่อแล้ว · { $count } โมเดล
settings-ai-connected-none = เชื่อมต่อแล้ว แต่ไม่มีรายชื่อโมเดล โปรดพิมพ์ชื่อโมเดล
settings-chatgpt-connected-none = เชื่อมต่อแล้ว โปรดพิมพ์ชื่อโมเดล
settings-ai-takes-images = รับรูปภาพได้
settings-ai-saved = การติดแท็กด้วย AI: { $service }
settings-ai-saved-off = ปิดการติดแท็กด้วย AI แล้ว
settings-ai-reset = การติดแท็กด้วย AI: กลับเป็นค่าเริ่มต้น
settings-ai-on = เปิด: { $service } · { $model }
settings-ai-url = ที่อยู่
settings-ai-key = คีย์ API
settings-ai-key-saved = บันทึกแล้ว (พิมพ์เพื่อแทนที่)
settings-ai-key-paste = วางคีย์ของคุณ
settings-ai-key-optional = ไม่มี (ไม่บังคับ)
settings-ai-key-remove = นำออก
settings-ai-get-key = รับคีย์
settings-ai-model = โมเดล
settings-ai-model-pick = เลือกหรือพิมพ์ชื่อโมเดล
settings-ai-model-name = ชื่อโมเดล
settings-ai-refresh = รีเฟรช
settings-ai-connect = เชื่อมต่อ
settings-ai-need-key = เพิ่มคีย์ API เพื่อดูโมเดล
settings-ai-vision = ส่งภาพย่อไปด้วย (ต้องใช้โมเดลที่รับรูปภาพได้)
settings-ai-batch = ต่อคำขอ
settings-ai-batch-unit = แอสเซ็ต (น้อยลงสำหรับโมเดลในเครื่องขนาดเล็ก มากขึ้นสำหรับโมเดลที่โฮสต์ไว้ขนาดใหญ่)

## SettingsPage.vue: ChatGPT sign-in

settings-chatgpt-account = บัญชี
settings-chatgpt-checking = กำลังตรวจสอบ…
settings-chatgpt-status-as = ลงชื่อเข้าใช้ในชื่อ { $email }
settings-chatgpt-status = ลงชื่อเข้าใช้แล้ว
settings-chatgpt-signed-in-as = ลงชื่อเข้าใช้ ChatGPT ในชื่อ { $email } แล้ว
settings-chatgpt-signed-in = ลงชื่อเข้าใช้ ChatGPT แล้ว
settings-chatgpt-signed-out = ลงชื่อออกจาก ChatGPT แล้ว
settings-chatgpt-continue = ดำเนินการต่อด้วย ChatGPT
settings-chatgpt-sign-out = ลงชื่อออก
settings-chatgpt-sign-out-title = ลงชื่อออกจาก ChatGPT หรือไม่?
settings-chatgpt-sign-out-in-use = การติดแท็กด้วย AI ใช้แพ็กเกจ ChatGPT ของคุณ จึงจะหยุดทำงานจนกว่าคุณจะลงชื่อเข้าใช้อีกครั้งหรือเลือกบริการอื่น
settings-chatgpt-sign-out-body = Hoard จะลืมการลงชื่อเข้าใช้และแจ้งให้ OpenAI ยุติการลงชื่อเข้าใช้นั้น
settings-chatgpt-manage = จัดการการใช้งาน
settings-chatgpt-finish-before = ลงชื่อเข้าใช้ให้เสร็จในหน้าต่างที่เปิดขึ้น (
settings-chatgpt-finish-link = เปิดอีกครั้ง
settings-chatgpt-finish-middle = ) หากสุดท้ายไปจบที่หน้าที่โหลดไม่ขึ้น ซึ่งมีที่อยู่ขึ้นต้นด้วย{ " " }
settings-chatgpt-finish-after = { " " }ให้คัดลอกที่อยู่นั้นมาวางที่นี่:
settings-chatgpt-paste = ที่อยู่ที่การลงชื่อเข้าใช้ไปจบ
settings-chatgpt-finish = เสร็จสิ้น
settings-chatgpt-usage = คำขอติดแท็กจะนับรวมในการใช้งานของแพ็กเกจ ChatGPT ของคุณ จัดการการใช้งาน จะแสดงสิ่งที่ Hoard ใช้ไปและให้คุณจำกัดได้ หากต้องการตัดการเชื่อมต่อ Hoard จากตัว ChatGPT เอง ให้ไปที่ Settings → Security and login → Sign in with ChatGPT บน chatgpt.com

## SettingsPage.vue: Local models

settings-local-downloading-progress = กำลังดาวน์โหลด { $percent }% · { $done } จาก { $total }
settings-local-downloading = กำลังดาวน์โหลด…
settings-local-incomplete = การดาวน์โหลดหยุดลง
settings-local-failed = เริ่มไม่สำเร็จเมื่อครั้งก่อน
settings-local-in-use-running = ใช้งานอยู่ · กำลังทำงาน
settings-local-in-use = ใช้งานอยู่
settings-local-starting = กำลังเริ่ม…
settings-local-running = กำลังทำงาน
settings-local-ready-images = พร้อม · รับรูปภาพได้
settings-local-ready-text = พร้อม · ข้อความเท่านั้น
settings-local-delete-title = ลบ { $name } หรือไม่?
settings-local-delete-in-use = การติดแท็กด้วย AI ใช้โมเดลนี้ จึงจะหยุดทำงานจนกว่าคุณจะเลือกโมเดลอื่น ไฟล์ของโมเดลจะถูกลบออกจากเซิร์ฟเวอร์โมเดล
settings-local-delete-downloading = การดาวน์โหลดจะหยุด และส่วนที่ดาวน์โหลดมาแล้วจะถูกลบ
settings-local-delete-body = ไฟล์ของโมเดลจะถูกลบออกจากเซิร์ฟเวอร์โมเดล คุณดาวน์โหลดอีกครั้งได้ภายหลัง
settings-local-delete = ลบ
settings-local-deleting = กำลังลบ…
settings-local-cancel = ยกเลิก
settings-local-resume = ดาวน์โหลดต่อ
settings-local-download = ดาวน์โหลด
settings-local-start-before = เริ่มด้วย{ " " }
settings-local-start-or = { " " }(หรือ{ " " }
settings-local-start-developing = { " " }ขณะพัฒนา) แล้ว{ " " }
settings-local-start-retry = ลองใหม่
settings-local-start-after = { " " }เซิร์ฟเวอร์ที่เรียกใช้โมเดลเดียวก็ใช้ได้เช่นกัน: ระบุชื่อโมเดลนั้นด้านล่าง
settings-local-list = บนเซิร์ฟเวอร์โมเดล
settings-local-none = ยังไม่ได้ดาวน์โหลดอะไร เลือกหนึ่งรายการด้านล่าง โมเดลจะเก็บไว้บนเซิร์ฟเวอร์ และคุณสลับได้ทุกเมื่อ
settings-local-catalog = โมเดลที่ดาวน์โหลดได้
settings-local-size = { $size } GB · { $license }
settings-local-other = โมเดลอื่นจาก Hugging Face
settings-local-other-placeholder = โมเดลอื่นจาก Hugging Face: owner/name-GGUF:Q4_K_M

## SettingsPage.vue: Adding packs

settings-store-pages = หน้าร้านค้า
settings-store-pages-what = แพ็กใหม่จะถูกค้นหาในร้านค้าในเบื้องหลัง เพื่อให้ได้หน้าร้านค้า ผู้ขาย และสัญญาอนุญาต
settings-store-lookup = ค้นหาหน้าร้านค้าโดยอัตโนมัติ
settings-unreal = แพ็กเกจ Unreal
settings-unreal-reset = แพ็กเกจ Unreal: กลับเป็นค่าเริ่มต้น
settings-unreal-what = สิ่งที่จะทำกับไฟล์ .uasset
settings-conv = กฎการแปลง
settings-conv-what = แปลงไฟล์ที่ Hoard เก็บไว้ให้เป็นรูปแบบเดียว เช่น แปลง OBJ และ FBX ทั้งหมดเป็น GLB เพื่อให้ทั้งคลังเป็นแบบเดียวกัน ไฟล์ที่แปลงแล้วจะแทนที่ต้นฉบับ โดยคงชื่อ แท็ก คอลเลกชัน และลิงก์ไว้ ไฟล์ในโฟลเดอร์ที่เฝ้าดูจะไม่ถูกเปลี่ยนเลย
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = เปิด
settings-conv-to = แปลงเป็น
# Heading over the file formats a rule converts.
settings-conv-from = จาก
settings-conv-remove = นำออก
settings-conv-remove-label = นำกฎข้อที่ { $n } ออก
settings-conv-taken = มีกฎอื่นแปลงรูปแบบนี้อยู่แล้ว
settings-conv-upright = ตั้งโมเดลให้ตรงขณะแปลง (Y ขึ้น หันไปทาง +Z) แบบเดียวกับที่แสดงโมเดลของแพ็ก
settings-conv-flac-what = เสียงยังเหมือนเดิมทุกประการ ในพื้นที่ราวครึ่งหนึ่ง และทุกอย่างใน WAV (จุดวนซ้ำ เครื่องหมาย cue แท็ก) จะเก็บไว้ใน FLAC จึงนำ WAV กลับมาได้ตรงตามเดิมทุกเมื่อ WAV ที่มีตัวอย่างแบบจุดลอยตัวหรือ 32 บิตจะยังเป็น WAV
settings-conv-add = เพิ่มกฎ
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = เก็บต้นฉบับไว้
settings-conv-keep-days =
    { $count ->
       *[other] วัน
    }
settings-conv-keep-what = ระหว่างนี้ หน้าของไฟล์ที่แปลงแล้วสามารถนำต้นฉบับกลับมาได้ เท็กซ์เจอร์จะไม่ถูกคัดลอกเข้าไปในโมเดลที่แปลงแล้ว โมเดลจะชี้ไปยังไฟล์ภาพที่อยู่ข้างๆ ส่วนไฟล์ .mtl หรือ .bin ของโมเดลยังคงอยู่
settings-conv-save = บันทึกกฎ
settings-conv-revert = เลิกทำการเปลี่ยนแปลง
settings-conv-existing = แปลงไฟล์ที่อยู่ในคลังแล้ว
settings-conv-saved = บันทึกกฎการแปลงแล้ว กฎจะใช้กับแพ็กที่เพิ่มต่อจากนี้
settings-conv-converting =
    { $count ->
       *[other] กำลังแปลงไฟล์ { $count } ไฟล์ในเบื้องหลัง
    }
settings-conv-nothing = ไม่มีอะไรในคลังที่ต้องแปลง
settings-conv-ask-title =
    { $count ->
       *[other] แปลงไฟล์ { $count } ไฟล์ที่อยู่ในคลังแล้วหรือไม่?
    }
settings-conv-ask-body =
    { $packs ->
       *[other] ไฟล์เหล่านี้อยู่ใน { $packs } แพ็ก
    } ต้นฉบับแต่ละไฟล์จะถูกเก็บไว้ { $days } วัน และระหว่างนั้นนำกลับมาได้จากหน้าของไฟล์
settings-conv-ask-busy =
    { $count ->
       *[other] { $count } ไฟล์อยู่ในแพ็กที่ยังเพิ่มไม่เสร็จ จะถูกแปลงเมื่อเพิ่มเสร็จแล้ว
    }
settings-conv-ask-list = ไฟล์ที่จะถูกแปลง
settings-conv-ask-more = …และอีก { $count } ไฟล์
settings-conv-ask-skipped = ปล่อยไว้ตามเดิม:
settings-conv-skip-watched =
    { $count ->
       *[other] { $count } ไฟล์ในโฟลเดอร์ที่เฝ้าดู
    }
settings-conv-skip-unity =
    { $count ->
       *[other] { $count } ไฟล์ที่โปรเจกต์ Unity อ้างอิงตามชื่อ
    }
settings-conv-skip-used =
    { $count ->
       *[other] { $count } ไฟล์ที่ฉากหรือไฟล์อื่นอ้างถึงตามชื่อ
    }
settings-conv-skip-beside =
    { $count ->
       *[other] { $count } ไฟล์ที่มีสำเนาที่แปลงแล้วอยู่ในแพ็กของไฟล์อยู่แล้ว
    }
settings-conv-skip-blender =
    { $count ->
       *[other] { $count } ไฟล์ที่ Blender ยังไม่ได้อ่าน (การตั้งค่า → เครื่องมือ)
    }
settings-conv-ask-new = บันทึก ใช้กับแพ็กใหม่เท่านั้น
settings-conv-ask-save-convert =
    { $count ->
       *[other] บันทึกและแปลง { $count } ไฟล์
    }
settings-conv-ask-convert =
    { $count ->
       *[other] แปลง { $count } ไฟล์
    }
settings-unreal-off = ปล่อยไว้ตามเดิม
settings-unreal-off-hint = แปลงจากหน้าของแพ็กเมื่อคุณต้องการ
settings-unreal-keep = แปลงและเก็บต้นฉบับไว้
settings-unreal-keep-hint = เท็กซ์เจอร์ เสียง และแมทีเรียลจะออกมาในรูปแบบทั่วไป และยังเก็บ .uasset ไว้ด้วย
settings-unreal-replace = แปลงและทิ้งต้นฉบับ
settings-unreal-replace-hint = ประหยัดพื้นที่เมื่อดึงเนื้อหาออกมาแล้ว เมชยังคงต้องรอตัวทำงาน Unreal

## SettingsPage.vue: Downloads

settings-tn = ชื่อเท็กซ์เจอร์
settings-tn-saved = บันทึกชื่อเท็กซ์เจอร์แล้ว
settings-tn-reset = ชื่อเท็กซ์เจอร์: กลับเป็นค่าเริ่มต้น
settings-tn-what = แมปของแมทีเรียลจะถูกส่งออกไปโดยตั้งชื่อตามแมทีเรียลและหน้าที่ของแต่ละแมป ไม่ว่าไฟล์จะชื่ออะไร: Rock_Normal.png ไม่ใช่ rock_moss_xeokah0_raw_8K_Normal.png ไฟล์ในคลังจะไม่ถูกแตะต้อง และแมปที่โมเดลหรือ MTL ที่ส่งไปด้วยระบุชื่อไว้จะคงชื่อนั้น เพื่อให้โมเดลยังหาแมปเจอ
settings-tn-toggle = ตั้งชื่อแมปเท็กซ์เจอร์ตามแมทีเรียล
settings-tn-style = รูปแบบ
settings-tn-style-group = รูปแบบการตั้งชื่อ
settings-tn-descriptive = แบบอธิบาย
settings-tn-short = แบบสั้น
settings-tn-custom = แบบของคุณเอง
settings-tn-pattern = รูปแบบชื่อ
settings-tn-hint-name = { " " }คือแมทีเรียล{ " " }
settings-tn-hint-map = { " " }คือหน้าที่ของแมป{ " " }
settings-tn-hint-size = { " " }คือขนาดเมื่อชุดนั้นมีมากกว่าหนึ่งขนาด (จะต่อท้ายให้หากรูปแบบไม่ได้ใส่ไว้)
settings-tn-example = ตัวอย่าง
settings-tn-suffixes = คำต่อท้ายสำหรับแมปแต่ละชนิด
settings-tn-slots = ในการดาวน์โหลด GLB เท็กซ์เจอร์ที่ฝังอยู่ในโมเดลโดยไม่มีไฟล์ของตัวเองจะได้ชื่อตามแมทีเรียลและสล็อตที่เท็กซ์เจอร์นั้นอยู่ และสล็อตเหล่านี้ก็มีคำต่อท้ายด้วย:
settings-tn-endings = ท้ายชื่อที่กำหนดเอง
settings-tn-endings-what = เท็กซ์เจอร์ที่ไม่ใช่แมปของแมทีเรียลจะคงชื่อเดิมไว้ เว้นแต่ชื่อจะลงท้ายด้วยรายการใดรายการหนึ่งเหล่านี้: _diff → _BC จะส่ง rock_diff.png เป็น rock_BC.png ปล่อยช่องที่สองว่างไว้เพื่อตัดท้ายชื่อนั้นออก
settings-tn-ending-from = ท้ายชื่อ
settings-tn-ending-from-example = _diff
settings-tn-ending-to = เปลี่ยนเป็น
settings-tn-ending-to-example = _BC
settings-tn-ending-add = เพิ่มท้ายชื่อ
settings-tn-ending-remove = นำออก
settings-tn-ending-remove-label = นำ { $ending } ออก

## SettingsPage.vue: General and Connection

settings-name = ชื่อ
settings-name-what = ชื่อที่เรียกเซิร์ฟเวอร์นี้ในแถบด้านบน แท็บเบราว์เซอร์ และหน้าลงชื่อเข้าใช้ เพื่อให้แยกได้ว่าเป็นคลังไหนเมื่อมีมากกว่าหนึ่งแห่ง
settings-name-what-desktop = ชื่อที่เรียกคลังนี้ในแถบด้านบน
settings-address = ที่อยู่
settings-address-what = ที่อยู่ที่ผู้คนและแอปใช้เข้าถึง Hoard เมื่อไม่ใช่ที่อยู่ที่เบราว์เซอร์นี้ใช้อยู่: ที่อยู่ของ reverse proxy, อุโมงค์ หรือชื่อใน tailnet เช่น https://hoard.example.com ลิงก์ที่ Hoard ส่งออกไป (ให้ Claude และแอปอื่น ในการลงชื่อเข้าใช้ บนใบรับรอง) จะใช้ที่อยู่นี้
settings-public-url = ที่อยู่สาธารณะ
settings-public-url-placeholder = จากที่อยู่ของเบราว์เซอร์ (เช่น https://hoard.example.com)
settings-public-url-default = จากที่อยู่ของเบราว์เซอร์

## ProfilerPanel.vue: the profiler at the top of Settings → Performance

profiler = โปรไฟเลอร์
profiler-what = สิ่งที่ Hoard ทำในช่วงสิบนาทีที่ผ่านมา อัปเดตทุกสองวินาทีขณะที่คุณดูอยู่ เก็บไว้ในหน่วยความจำเท่านั้นและเริ่มใหม่เมื่อ Hoard รีสตาร์ต
profiler-starting = ค่าที่วัดได้ชุดแรกจะมาถึงในอีกสักครู่
profiler-failed = อ่านโปรไฟเลอร์ไม่ได้: { $error }
profiler-cpu = การใช้โปรเซสเซอร์
profiler-cpu-hoard = Hoard
profiler-cpu-machine = ทั้งเครื่อง
profiler-cpu-summary = Hoard ใช้โปรเซสเซอร์ { $hoard } และทั้งเครื่องใช้ { $machine }
profiler-ram = RAM
profiler-ram-peak = สูงสุด
profiler-ram-of = { $used } จาก { $total }
profiler-ram-summary = Hoard ใช้ RAM อยู่ { $hoard } และใช้สูงสุด { $peak } ในสิบนาทีที่ผ่านมา
profiler-serving = คำขอที่กำลังดำเนินการ
profiler-working = กำลังทำงาน
profiler-working-none = ไม่มีงานใดทำงานในเบื้องหลัง
profiler-working-more =
    { $count ->
       *[other] และอีก { $count } รายการ
    }
profiler-requests = เวลาที่ใช้ให้บริการคำขอ
profiler-requests-what = แต่ละแท่งคือคำขอในช่วงสิบวินาที โดยแบ่งเวลาตามส่วนที่เวลาถูกใช้ไป
profiler-part-db = ฐานข้อมูล
profiler-part-db-title = รอฐานข้อมูลและเรียกใช้คิวรีของฐานข้อมูล
profiler-part-disk = ดิสก์
profiler-part-disk-title = อ่านไฟล์ที่จัดเก็บไว้ และบีบอัดสิ่งที่ส่ง
profiler-part-send = การส่ง
profiler-part-send-title = รอขณะที่เครือข่ายรับสิ่งที่ส่งไป
profiler-part-other = อื่น ๆ
profiler-part-other-title = เตรียมคำตอบ
profiler-request-count =
    { $count ->
       *[other] { $count } คำขอ
    }
profiler-requests-none = ไม่มีคำขอในสิบนาทีที่ผ่านมา
profiler-slowest = ช้าที่สุด: { $route }, { $time }
profiler-routes = ตามเส้นทาง
profiler-route = เส้นทาง
profiler-count = คำขอ
profiler-average = เฉลี่ย
profiler-p95 = ช้าที่สุด 5%
profiler-split = เวลาหมดไปกับอะไร
profiler-sent = ส่งแล้ว
profiler-portal = ไฟล์ของอินเทอร์เฟซเอง

## SettingsPage.vue: Performance

settings-render = ภาพย่อ 3 มิติ
settings-render-toggle = เรนเดอร์ภาพย่อของโมเดล 3 มิติ
settings-gpu-on = บน GPU: { $renderer }
settings-gpu-software = ไม่พบ GPU จึงใช้ซอฟต์แวร์ ({ $renderer })
settings-gpu-none = ไม่พบ GPU หรือตัวเรนเดอร์แบบซอฟต์แวร์ โมเดลจึงไม่มีภาพย่อ
settings-thumbs = ภาพย่อ
settings-thumbs-what = จัดเก็บเป็น WebP แบบบีบอัดหรือแบบตรงต้นฉบับ แล้วแต่ว่าแบบไหนเล็กกว่า ภาพเล็กที่ไฟล์เดิมเล็กกว่าอยู่แล้วจะใช้ตัวเองเป็นภาพย่อ การเปลี่ยนตรงนี้จะสร้างภาพย่อที่มีอยู่ใหม่ในเบื้องหลัง
settings-previews-size = ภาพตัวอย่าง: { $size }
settings-thumbs-saved = ภาพย่อ: { $quality }
settings-thumbs-size = ขนาด
settings-thumbs-size-option = { $px } px
settings-thumbs-quality = คุณภาพ { $quality }
settings-thumbs-small = เล็กกว่า
settings-thumbs-small-hint = ดูดีในตาราง แต่จะนุ่มไปเล็กน้อยเมื่อดูใกล้ ๆ
settings-thumbs-balanced = สมดุล
settings-thumbs-balanced-hint = แยกจากต้นฉบับได้ยากที่ขนาดภาพย่อ
settings-thumbs-sharp = คมชัดกว่า
settings-thumbs-sharp-hint = ขนาดประมาณสองเท่าของแบบสมดุล
settings-motion-thumbs = ภาพย่อแบบเคลื่อนไหวสำหรับแอนิเมชัน
settings-motion-thumbs-what = แต่ละแอนิเมชันจะมีวิดีโอสั้นที่เล่นวนด้วย โดยเล่นเมื่อคุณชี้ที่ไทล์ของมัน แต่ละอันใช้พื้นที่ราว 40 KB และใช้เวลาสร้างสักพักสำหรับคลังขนาดใหญ่ ปิดตัวเลือกนี้แล้วจะลบทิ้ง
settings-motion-fps = อัตราเฟรม
settings-motion-fps-option = { $fps } fps
settings-motion-fps-what = ยิ่งลื่นไหลยิ่งใช้เวลาสร้างนานและใช้พื้นที่มากขึ้น ที่ 24 fps ใช้ประมาณสองเท่าของ 12 และที่ 30 ใช้มากกว่านั้นอีก เมื่อเปลี่ยนค่านี้ ระบบจะสร้างใหม่ทั้งหมด
settings-limits = งานและขีดจำกัด
settings-limits-what = แต่ละค่าจะบันทึกเมื่อคุณออกจากช่องหรือกด Enter ขนาดต้องมีหน่วย: 100 GB, 512 MB, 1.5 TB ตัวเลขเปล่า ๆ หมายถึง GB
settings-workers = ตัวทำงานเบื้องหลัง
settings-workers-unit = งานพร้อมกัน (นำเข้า ติดแท็ก แปลง)
settings-max-upload = อัปโหลดใหญ่สุด
settings-max-upload-unit = ต่อไฟล์
settings-max-expand = แพ็กใหญ่สุด
settings-max-expand-unit = เมื่อแตกไฟล์แล้ว
settings-conversion-cache = ไฟล์ดาวน์โหลดที่แปลงแล้ว
settings-conversion-cache-unit-now = เก็บไว้ไม่เกินนี้ ตอนนี้ { $size } รายการที่ไม่ได้ใช้นานที่สุดจะถูกลบก่อน
settings-conversion-cache-unit = เก็บไว้ไม่เกินนี้ รายการที่ไม่ได้ใช้นานที่สุดจะถูกลบก่อน

## SettingsPage.vue: Maintenance

settings-updates = การอัปเดต
settings-version = นี่คือ Hoard { $version }
settings-update-check = ตรวจหารุ่นใหม่บน GitHub วันละครั้ง
settings-desktop-updates = Hoard จะหารุ่นใหม่ทุกครั้งที่เปิด ดาวน์โหลดไว้เบื้องหลัง และติดตั้งเมื่อคุณปิดโปรแกรม
settings-backups = การสำรองข้อมูลตามกำหนดเวลา
settings-backups-what = ฐานข้อมูลจะสำรองตัวเองโดยอัตโนมัติ ข้อมูลสำรองแต่ละชุดจะแสดงในประวัติด้านล่าง
settings-backup-every = สำรองข้อมูล
settings-backup-off = ไม่เลย
settings-backup-daily = ทุกวัน
settings-backup-weekly = ทุกสัปดาห์
settings-backup-keep = เก็บไว้
settings-backup-keep-unit = ชุดล่าสุดที่สำรองตามกำหนดเวลา
settings-backup-copy-to = คัดลอกไปที่
settings-backup-copy-to-none = ไม่มีโฟลเดอร์อื่น
settings-backups-kept = แต่ละชุดจะอยู่ในโฟลเดอร์สำรองข้อมูลชื่อ hoard-auto-…db และในโฟลเดอร์อื่น (ดิสก์อื่น หรือโฟลเดอร์ที่แอปซิงค์สำรองไว้) ถ้ามี โดยระบุด้วยพาธเต็มบนเซิร์ฟเวอร์ ระบบจะลบเฉพาะข้อมูลสำรองตามกำหนดเวลาที่เก่ากว่าเท่านั้น ส่วนที่ทำเองจะยังอยู่
about-what = Hoard เก็บแอสเซ็ตเกม (โมเดล เท็กซ์เจอร์ เสียง เพลง สไปรต์ และอื่น ๆ) ไว้ในคลังเดียวที่คุณค้นหา ดูตัวอย่าง และหยิบไปใช้ได้ โดยแชร์กับทุกคนที่คุณให้สิทธิ์เข้าถึง
about-version = รุ่น
about-edition = ฉบับ
about-edition-server = เซิร์ฟเวอร์
about-edition-desktop = แอปเดสก์ท็อป
about-edition-steam = แอปเดสก์ท็อปจาก Steam
about-made-by = สร้างโดย
about-source = ซอร์สโค้ดบน GitHub
about-issues = รายงานปัญหา
about-discord = คุยกับเราบน Discord
about-steam = Hoard บน Steam
about-license = สัญญาอนุญาต
about-license-what = Hoard เป็นโอเพนซอร์สภายใต้สัญญาอนุญาต MIT No Attribution: ใช้ แก้ไข และแบ่งปันได้ตามต้องการ
about-downloaded = ดาวน์โหลดเมื่อจำเป็น
about-downloaded-what = Hoard ไม่ได้มาพร้อมสิ่งเหล่านี้ แต่จะดาวน์โหลดแต่ละรายการเมื่อมีสิ่งใดต้องใช้เป็นครั้งแรก และแต่ละรายการมีสัญญาอนุญาตของตัวเอง
about-blender = แปลงโมเดล 3D และสร้างภาพขนาดย่อ
about-ffmpeg = แปลงเสียงและวิดีโอ และสร้างตัวอย่าง
about-siglip = ค้นหารูปภาพจากสิ่งที่อยู่ในภาพ
about-clap = ค้นหาเสียงจากลักษณะที่ได้ยิน
about-models = โมเดลที่คุณเลือกสำหรับเซิร์ฟเวอร์ AI ในเครื่องอยู่ภายใต้สัญญาอนุญาตของผู้สร้าง
about-built-with = สร้างด้วย
about-built-with-what = ไลบรารีโอเพนซอร์สที่ใช้สร้าง Hoard พร้อมสัญญาอนุญาต
about-steamworks = รุ่น Steam มี Steamworks SDK ด้วย © Valve Corporation
about-licenses-loading = กำลังโหลดรายการ…
about-licenses-failed = โหลดรายการไม่ได้
about-filter = กรองตามชื่อหรือสัญญาอนุญาต
about-count = { $shown } จาก { $total }
about-no-license = ไม่ได้ระบุสัญญาอนุญาต
about-report = รายงานปัญหา…
report-title = รายงานปัญหา
report-what = นี่คือสิ่งที่ issue บน GitHub ต้องใช้เพื่อตรวจสอบปัญหา ไม่มีข้อมูลส่วนตัวอยู่ในนี้ โทเค็นการเข้าถึง รหัสผ่าน คีย์ ที่อยู่อีเมล และชื่อของคุณในพาธโฟลเดอร์จะถูกนำออก อ่านทวนก่อนส่ง
report-loading = กำลังรวบรวมรายงาน…
report-failed = รวบรวมรายงานไม่ได้: { $error }
report-log = สิ่งที่เซิร์ฟเวอร์บันทึกไว้ล่าสุด
report-copy = คัดลอก
report-copied = คัดลอกรายงานแล้ว
report-github = เปิด issue บน GitHub
report-github-hint = issue ใหม่จะถูกกรอกด้วยข้อมูลนี้ ถ้าบันทึกยาวเกินไป จะใส่เฉพาะบรรทัดท้ายๆ ส่วนคัดลอกมีครบทั้งหมด
report-close = ปิด

## Asset kinds (api.ts)

api-kind-texture = เท็กซ์เจอร์
api-kind-vector = ภาพเวกเตอร์
api-kind-sheet = แผ่นสไปรต์
api-kind-audio = เสียง
api-kind-model = โมเดล
api-kind-animation = แอนิเมชัน
api-kind-font = ฟอนต์
api-kind-shader = เชดเดอร์
api-kind-material = แมทีเรียล
api-kind-video = วิดีโอ
api-kind-document = เอกสาร
api-kind-data = ข้อมูล
api-kind-other = อื่น ๆ
api-kind-texture-name = เท็กซ์เจอร์
api-kind-vector-name = ภาพเวกเตอร์
api-kind-sheet-name = แผ่นสไปรต์
api-kind-audio-name = เสียง
api-kind-model-name = โมเดล
api-kind-animation-name = แอนิเมชัน
api-kind-font-name = ฟอนต์
api-kind-shader-name = เชดเดอร์
api-kind-material-name = แมทีเรียล
api-kind-video-name = วิดีโอ
api-kind-document-name = เอกสาร
api-kind-data-name = ข้อมูล
api-kind-other-name = ประเภทอื่น

## Toasts, dialogs and loading (ui.ts, UiLayer.vue, lazy.ts)

news-button = ใหม่ { $count } รายการ
news-button-title =
    { $count ->
       *[other] สิ่งใหม่ใน Hoard { $count } รายการ
    }
news-pip =
    { $count ->
        [1] ใหม่ใน Hoard
       *[other] มีสิ่งใหม่ { $count } รายการที่นี่
    }
news-title = ใหม่ใน Hoard
news-to-find =
    { $count ->
       *[other] รอให้ค้นพบ { $count } รายการ
    }
news-close = ปิด
news-got-it = รับทราบ
news-take-me = พาไปที่นั่น
news-look-again = ดูอีกครั้ง
news-none = ตอนนี้ยังไม่มีอะไรใหม่ให้ค้นพบ
news-seen-lately = เห็นเมื่อเร็ว ๆ นี้
news-stop-pointing = หยุดชี้สิ่งใหม่
news-point-out = กลับมาชี้สิ่งใหม่อีกครั้ง
news-mark-all = ทำเครื่องหมายว่าเห็นทั้งหมดแล้ว
news-marked-seen =
    { $count ->
       *[other] ทำเครื่องหมายว่าเห็นแล้ว { $count } รายการ
    }
news-themes-title = ธีมสว่างสิบแบบและธีมมืดสิบแบบ
news-themes-body = Sky, Mint, Apricot และ Lemon มาร่วมกับธีมสว่าง ส่วน Midnight, Lagoon และ Neon มาร่วมกับธีมมืด และรูปลักษณ์ใหม่จะแผ่ออกจากไทล์ที่คุณเลือก
news-tiles-move-title = ให้ไทล์เคลื่อนไหวตลอด
news-tiles-move-body = แอนิเมชันและภาพเคลื่อนไหวบนไทล์ในคลังเล่นได้ตลอดเวลา ไม่ใช่แค่ตอนที่ตัวชี้อยู่เหนือภาพ
news-motion-trails-title = เส้นทางการเคลื่อนไหวบนแอนิเมชัน
news-motion-trails-body = ภาพย่อของแอนิเมชันจะลากเส้นทางที่ตัวละครเคลื่อนที่ไป พร้อมจุดบอกตำแหน่งปัจจุบัน
news-text-size-title = ขนาดข้อความและระยะห่าง
news-text-size-body = ปรับตัวหนังสือให้ใหญ่ขึ้นหรือเล็กลง ปรับหน้าให้โปร่งขึ้นหรือกะทัดรัดขึ้น และเลือกสิ่งที่ไทล์และแถบด้านบนแสดง
news-tag-tree-title = แท็กซ้อนแท็ก
news-tag-tree-body = ใส่แท็กไว้ใต้แท็กที่กว้างกว่า เพื่อให้ค้นหา “ยานพาหนะ” แล้วเจอ “รถยนต์” และ “รถเก๋ง” ด้วย จัดซ้อนกันได้ที่ การตั้งค่า → แท็ก หรือในหน้าของแต่ละแท็ก
news-bulk-edit-title = แก้ไขหลายรายการพร้อมกัน
news-bulk-edit-body = เลือกแอสเซ็ตแล้วกด E เพื่อเปลี่ยนแท็ก สัญญาอนุญาต คะแนน และอื่น ๆ พร้อมกัน โดยเลิกทำได้
news-backups-title = การสำรองข้อมูลตามกำหนดเวลา
news-backups-body = สำรองฐานข้อมูลทุกวันหรือทุกสัปดาห์ เก็บชุดล่าสุดไว้ไม่กี่ชุด และคัดลอกแต่ละชุดไปยังโฟลเดอร์อื่น
ui-undo = เลิกทำ
ui-today = วันนี้
ui-yesterday = เมื่อวาน
ui-days-ago = { $days } วันที่แล้ว
ui-notifications = การแจ้งเตือน
ui-dismiss = ปิด
ui-cancel = ยกเลิก
ui-lazy-loading = กำลังโหลด{ $what }…
ui-lazy-failed = โหลด{ $what }ไม่ได้
ui-lazy-reload = โหลดหน้านี้ใหม่

## ResetButton.vue

reset-title = รีเซ็ตเป็นค่าเริ่มต้น: { $value }
reset-label = รีเซ็ตเป็นค่าเริ่มต้น { $value }

## SizeInput.vue

size-invalid = ขนาดเช่น 100 GB, 512 MB หรือ 1.5 TB
size-hint = ขนาดเช่น 100 GB, 512 MB หรือ 1.5 TB ตัวเลขเปล่า ๆ หมายถึง GB

## AuthPanel.vue (Settings → Authentication)

auth-load-failed = โหลดการตั้งค่าการยืนยันตัวตนไม่ได้: { $error }
auth-saved = บันทึกแล้ว มีผลกับการลงชื่อเข้าใช้ครั้งถัดไป
auth-method-internal = ภายใน
auth-method-internal-hint = ชื่อและรหัสผ่านที่ Hoard เก็บไว้เอง เพิ่มได้ที่ การตั้งค่า → บัญชี
auth-method-ldap-hint = ไดเรกทอรีจะตรวจสอบชื่อและรหัสผ่านที่พิมพ์ในหน้าลงชื่อเข้าใช้: OpenLDAP, FreeIPA, LLDAP หรือเซิร์ฟเวอร์ LDAP อื่น
auth-method-ad-hint = บัญชีโดเมน Windows ลงชื่อเข้าใช้ด้วยชื่อและรหัสผ่านที่ใช้ตามปกติ
auth-method-oauth-hint = ปุ่ม “ลงชื่อเข้าใช้ด้วย…” ที่พาไปยัง Google, Microsoft, GitHub, Keycloak, Authentik หรือผู้ให้บริการอื่นแล้วกลับมา
auth-session-unused = หลังจากไม่ได้ใช้ 30 วัน
auth-session-1h = 1 ชั่วโมงหลังลงชื่อเข้าใช้
auth-session-8h = 8 ชั่วโมงหลังลงชื่อเข้าใช้
auth-session-day = หนึ่งวันหลังลงชื่อเข้าใช้
auth-session-week = หนึ่งสัปดาห์หลังลงชื่อเข้าใช้
auth-session-30d = 30 วันหลังลงชื่อเข้าใช้
auth-session-hours = { $hours } ชั่วโมงหลังลงชื่อเข้าใช้
auth-role-view = ดูได้
auth-role-edit = แก้ไขได้
auth-role-admin = ผู้ดูแลระบบ
auth-role-word-view = ดูได้
auth-role-word-edit = แก้ไขได้
auth-role-word-admin = ผู้ดูแลระบบ
auth-role-word-default = { $role } (บทบาทเริ่มต้น)
auth-tls-starttls-hint = LDAP ธรรมดา (พอร์ต 389) ที่สลับเป็น TLS ก่อนส่งข้อมูลใด ๆ
auth-tls-ldaps-hint = TLS ตั้งแต่เริ่มต้น (พอร์ต 636)
auth-tls-none = ไม่มี
auth-tls-none-hint = รหัสผ่านจะผ่านเครือข่ายตามที่เป็น ใช้เฉพาะบนเครือข่ายที่คุณไว้ใจเท่านั้น
auth-test-failed = ทดสอบไม่ได้: { $error }
auth-preset-custom = อื่น ๆ (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel และอื่น ๆ: ระบุผู้ออก (issuer) แล้ว ค้นหาอัตโนมัติ จะกรอกที่อยู่ให้
auth-preset-google-note = สร้าง OAuth client (Web application) ใน Google Cloud Console → APIs & Services → Credentials Google ไม่ส่งกลุ่มมา จึงต้องกำหนดบทบาทที่ การตั้งค่า → บัญชี และปิดการสร้างบัญชีเมื่อลงชื่อเข้าใช้ครั้งแรก เว้นแต่ทุกคนที่มีบัญชี Google ควรเข้าได้
auth-preset-microsoft-note = ลงทะเบียนแอปใน Entra ID → App registrations (ตั้ง redirect แบบ Web ไปยังที่อยู่ด้านล่าง) ใส่ tenant ID ของคุณแทน “organizations” เพื่อให้เฉพาะองค์กรของคุณเข้าได้ หากต้องการบทบาทจากกลุ่ม ให้เพิ่ม groups claim ใน Token configuration: ซึ่งจะระบุ ID ของกลุ่ม
auth-preset-github-note = สร้าง OAuth app ใน GitHub → Settings → Developer settings GitHub ไม่ส่งกลุ่มมา จึงต้องกำหนดบทบาทที่ การตั้งค่า → บัญชี และปิดการสร้างบัญชีเมื่อลงชื่อเข้าใช้ครั้งแรก เว้นแต่ทุกคนที่มีบัญชี GitHub ควรเข้าได้
auth-preset-gitlab-note = สร้างแอปพลิเคชันใน GitLab → Preferences → Applications (หรือในส่วนผู้ดูแลระบบของอินสแตนซ์ของคุณ: เปลี่ยน gitlab.com ในที่อยู่)
auth-discovered = กรอกที่อยู่ของผู้ให้บริการแล้ว
auth-copied = คัดลอกแล้ว
auth-copy-failed = คัดลอกไม่ได้: เลือกข้อความแล้วคัดลอกเอง
auth-sign-in-with = ลงชื่อเข้าใช้ด้วย
auth-own-accounts-still-work = บัญชีของ Hoard เองยังลงชื่อเข้าใช้ด้วยรหัสผ่านของตัวเองได้ ดังนั้นไดเรกทอรีหรือผู้ให้บริการที่ล่มหรือตั้งค่าผิดจะไม่ทำให้คุณเข้าไม่ได้
auth-sessions = เซสชัน
auth-sign-out-after = ลงชื่อออกให้ผู้ใช้
auth-ldap-server = เซิร์ฟเวอร์ LDAP
auth-server-type = ประเภทเซิร์ฟเวอร์
auth-server-type-custom = กำหนดเอง
auth-servers = เซิร์ฟเวอร์
auth-server-n = เซิร์ฟเวอร์ { $n }
auth-remove-server-n = นำเซิร์ฟเวอร์ { $n } ออก
auth-remove = นำออก
auth-add-server = เพิ่มเซิร์ฟเวอร์
auth-servers-hint = หากติดต่อเซิร์ฟเวอร์หนึ่งไม่ได้ จะลองเซิร์ฟเวอร์ถัดไป
auth-security = ความปลอดภัย
auth-connection-security = ความปลอดภัยของการเชื่อมต่อ
auth-skip-verify = ไม่ตรวจสอบใบรับรองของเซิร์ฟเวอร์ (ใช้เพื่อทดลองเท่านั้น)
auth-ca = ผู้ออกใบรับรอง
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (เว้นว่าง: ใช้ที่ระบบเชื่อถือ)
auth-lookup = การค้นหาผู้ใช้
auth-anonymously = แบบไม่ระบุตัวตน
auth-reader-account = บัญชีสำหรับอ่าน
auth-reader-password = รหัสผ่านของบัญชีนั้น
auth-saved-type-to-replace = บันทึกแล้ว: พิมพ์เพื่อแทนที่
auth-people-under = ผู้ใช้อยู่ภายใต้
auth-name-attribute = แอตทริบิวต์ชื่อ
auth-only-matching = เฉพาะรายการที่ตรงกับ
auth-groups-under = กลุ่มอยู่ภายใต้
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (เว้นว่าง: ใช้ memberOf เท่านั้น)
auth-their-groups = กลุ่มของผู้ใช้
auth-group-filter-hint = { "{" }dn{ "}" } คือรายการของผู้ใช้ { "{" }user{ "}" } คือชื่อที่ใช้ลงชื่อเข้าใช้
auth-test = ทดสอบ
auth-test-hint = ติดต่อแต่ละเซิร์ฟเวอร์และลงชื่อเข้าใช้ด้วยบัญชีสำหรับอ่าน หากระบุชื่อและรหัสผ่าน จะลงชื่อเข้าใช้ในชื่อผู้ใช้นั้นด้วยและแสดงว่า Hoard จะตีความอย่างไร ไม่มีการบันทึกใด ๆ
auth-test-user = ผู้ใช้ที่จะลงชื่อเข้าใช้
auth-test-user-placeholder = ผู้ใช้ที่จะลงชื่อเข้าใช้ (ไม่บังคับ)
auth-test-password = รหัสผ่านของผู้ใช้
auth-testing = กำลังทดสอบ…
auth-test-connectivity = ทดสอบการเชื่อมต่อ
auth-test-server-ok = ติดต่อได้ และบัญชีสำหรับอ่านลงชื่อเข้าใช้แล้ว
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }ลงชื่อเข้าใช้แล้ว (
auth-test-signed-in-after = )
auth-test-role = จะได้รับบทบาท: { $role }
auth-test-no-role = ผู้ใช้ไม่อยู่ในกลุ่มใดที่ระบุไว้ใต้ บทบาท จึงจะถูกปฏิเสธ
auth-test-groups = กลุ่ม: { $groups }
auth-test-no-groups = ไม่พบกลุ่ม
auth-provider = ผู้ให้บริการ
auth-redirect = ที่อยู่ redirect
auth-copy = คัดลอก
auth-redirect-hint = ให้ที่อยู่นี้กับผู้ให้บริการเมื่อคุณสร้าง client ของ Hoard ที่อยู่นี้มาจากที่อยู่สาธารณะใน การตั้งค่า → การเชื่อมต่อ หรือที่อยู่ที่คุณใช้อยู่
auth-issuer = ผู้ออก (Issuer)
auth-discover = ค้นหาอัตโนมัติ
auth-client-id = Client ID
auth-client-secret = Client secret
auth-authorization-url = ที่อยู่สำหรับการอนุญาต
auth-token-url = ที่อยู่โทเค็น
auth-userinfo-url = ที่อยู่ข้อมูลผู้ใช้
auth-userinfo-placeholder = (เว้นว่าง: ID token จะระบุว่าเป็นใคร)
auth-scopes = ขอบเขต (Scopes)
auth-name-from = ชื่อจาก
auth-groups-from = กลุ่มจาก
auth-groups-from-placeholder = (ไม่มี)
auth-button = ปุ่ม
auth-button-placeholder = บัญชีของคุณ
auth-accounts = บัญชี
auth-provision = สร้างบัญชี Hoard เมื่อมีคนลงชื่อเข้าใช้เป็นครั้งแรก
auth-provision-on-hint = มิฉะนั้นผู้ดูแลระบบต้องเพิ่มแต่ละคนก่อนที่ การตั้งค่า → บัญชี
auth-provision-off-hint = ผู้ดูแลระบบต้องเพิ่มแต่ละคนก่อนที่ การตั้งค่า → บัญชี โดยเลือกวิธีที่คนนั้นลงชื่อเข้าใช้
auth-open-to-anyone = ทุกคนที่มีบัญชี { $provider } เข้าได้ เพิ่มกลุ่มผู้ดูด้านล่าง หรือปิดตัวเลือกนี้แล้วเพิ่มผู้ใช้เอง
auth-new-accounts = บัญชีใหม่
auth-roles-from-groups = บทบาทจากกลุ่ม
auth-roles-hint = กลุ่มที่สมาชิกจะได้รับแต่ละบทบาท บรรทัดละหนึ่งกลุ่ม ตามชื่อหรือ DN บทบาทสูงสุดที่ตรงกันจะถูกใช้ และจะตรวจสอบทุกครั้งที่ลงชื่อเข้าใช้ หากไม่ระบุกลุ่มใด บัญชีใหม่จะได้รับบทบาทด้านบน และผู้ดูแลระบบเปลี่ยนได้ภายหลัง หากระบุกลุ่มไว้ใต้ ดูได้ เฉพาะสมาชิกของกลุ่มที่ระบุเท่านั้นที่ลงชื่อเข้าใช้ได้
auth-roles-hint-oauth = กลุ่มที่สมาชิกจะได้รับแต่ละบทบาท บรรทัดละหนึ่งกลุ่ม ตามชื่อหรือ DN (หรือสิ่งที่ groups claim ระบุ) บทบาทสูงสุดที่ตรงกันจะถูกใช้ และจะตรวจสอบทุกครั้งที่ลงชื่อเข้าใช้ หากไม่ระบุกลุ่มใด บัญชีใหม่จะได้รับบทบาทด้านบน และผู้ดูแลระบบเปลี่ยนได้ภายหลัง หากระบุกลุ่มไว้ใต้ ดูได้ เฉพาะสมาชิกของกลุ่มที่ระบุเท่านั้นที่ลงชื่อเข้าใช้ได้
auth-saving = กำลังบันทึก…
auth-save = บันทึก

## HttpsPanel.vue (Settings → Connection)

https-load-failed = โหลด HTTPS ไม่ได้: { $error }
https-off-redirecting = ปิด HTTPS แล้ว กำลังพาคุณไปที่ http://
https-now-on = เปิด HTTPS แล้ว
https-saved = บันทึกแล้ว การเชื่อมต่อใหม่จะใช้ใบรับรองใหม่
https-now-off = ปิด HTTPS แล้ว
https-http3-failed-toast = เริ่ม HTTP/3 ไม่ได้: { $error }
https-copied = คัดลอกแล้ว
https-copy-failed = คัดลอกไม่ได้: เลือกข้อความแล้วคัดลอกเอง
https-mode-off = ปิด
https-mode-off-hint = http:// ธรรมดาเท่านั้น
https-mode-own = ใบรับรองของ Hoard เอง
https-mode-own-hint = Hoard จะสร้างใบรับรองสำหรับที่อยู่ของเซิร์ฟเวอร์นี้ ติดตั้งผู้ออกใบรับรองของ Hoard หนึ่งครั้งบนแต่ละอุปกรณ์ แล้วเบราว์เซอร์ในอุปกรณ์นั้นจะเชื่อถือ
https-mode-custom = ใบรับรองของฉัน
https-mode-custom-hint = ใบรับรองที่คุณมีอยู่แล้ว เช่น จาก tailscale cert หรือ Let’s Encrypt เบราว์เซอร์จะเชื่อถือโดยไม่ต้องติดตั้งอะไร
https-untrusted-own = เบราว์เซอร์นี้ยังไม่เชื่อถือใบรับรองสำหรับ { $host }: ติดตั้งผู้ออกใบรับรองของ Hoard ด้านล่าง คุณจะเห็นคำเตือนที่คลิกผ่านได้
https-untrusted-uncovered = เบราว์เซอร์นี้ยังไม่เชื่อถือใบรับรองสำหรับ { $host }: ใบรับรองไม่ครอบคลุมที่อยู่นี้ คุณจะเห็นคำเตือนที่คลิกผ่านได้
https-untrusted = เบราว์เซอร์นี้ยังไม่เชื่อถือใบรับรองสำหรับ { $host } คุณจะเห็นคำเตือนที่คลิกผ่านได้
https-now = ตอนนี้
https-start-failed = ตั้งค่า HTTPS แล้วแต่เริ่มไม่ได้: { $error }
https-on-port = เปิด HTTPS แล้ว บนพอร์ต { $port } เดียวกับ http://
https-using-it = คุณกำลังใช้อยู่ตอนนี้
https-open = เปิด { $url }
https-trusted = เบราว์เซอร์นี้เชื่อถือใบรับรองนี้
https-cert-for-before = ใบรับรองสำหรับ{ " " }
https-cert-for-after = { " " }ใช้ได้ถึง { $date }
https-cert-for-after-own = { " " }ใช้ได้ถึง { $date } (ต่ออายุเอง)
https-not-covered-before = ใบรับรองไม่ครอบคลุม{ " " }
https-not-covered-after = { " " }ซึ่งเป็นที่อยู่ที่คุณใช้อยู่
https-is-off = ปิด HTTPS อยู่: หน้าเว็บให้บริการผ่าน http:// ธรรมดา
https-addresses = ที่อยู่
https-addresses-hint = ใบรับรองครอบคลุมที่อยู่ที่ Hoard พบ และที่อยู่ที่คุณเพิ่ม (ชื่อที่เราเตอร์ตั้งให้ ชื่อ Tailscale)
https-remove-name = นำ { $name } ออก
https-another-address = ที่อยู่อื่น
https-another-address-placeholder = ที่อยู่อื่น เช่น nas.local
https-add = เพิ่ม
https-your-cert = ใบรับรองของคุณ
https-saved-cert-before = บันทึกแล้ว:{ " " }
https-saved-cert-after = { " " }จาก { $issuer } ใช้ได้ถึง { $date } วางใบรับรองใหม่เพื่อแทนที่
https-pem-before = ใบรับรอง (พร้อมเชน) และคีย์ส่วนตัวในรูปแบบข้อความ PEM: ไฟล์ที่ขึ้นต้นด้วย{ " " }
https-pem-after = { "" }
https-certificate = ใบรับรอง
https-choose-file = เลือกไฟล์…
https-private-key = คีย์ส่วนตัว
https-checked-before = สำหรับ{ " " }
https-checked-after = { " " }จาก { $issuer } ใช้ได้ถึง { $date }
https-checked-after-days = { " " }จาก { $issuer } ใช้ได้ถึง { $date } ({ $days } วัน)
https-self-signed = เป็นใบรับรองที่ลงนามเอง: เบราว์เซอร์จะแสดงคำเตือน
https-key-matches = คีย์ตรงกัน
https-where-to-get = หาใบรับรองได้จากที่ใด
https-get-tailscale-before = เปิด HTTPS สำหรับ tailnet ของคุณ แล้วเรียกใช้{ " " }
https-get-tailscale-after = { " " }บนเซิร์ฟเวอร์และอัปโหลดทั้งสองไฟล์ ไฟล์ใช้ได้ 90 วัน: เรียกใช้อีกครั้งและอัปโหลดใหม่ก่อนหมดอายุ
https-get-acme-title = Let’s Encrypt หรือ ACME client อื่น
https-get-acme-before = (certbot, acme.sh, lego) สำหรับโดเมนที่คุณเป็นเจ้าของ: อัปโหลด{ " " }
https-get-acme-and = { " " }และ{ " " }
https-get-acme-after = { "" }
https-get-proxy-title = Reverse proxy
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) ที่มีใบรับรอง: ปิด HTTPS ที่นี่ไว้ แล้วให้พร็อกซีส่ง http:// ธรรมดามายัง Hoard
https-trying = กำลังทดลอง…
https-apply = ใช้
https-apply-hint = Hoard จะทดลองการตั้งค่าใหม่ก่อน และคงการตั้งค่าเดิมไว้หากใช้ไม่ได้
https-trust-title = ให้อุปกรณ์ของคุณเชื่อถือ Hoard
https-download-ca = ดาวน์โหลด hoard-ca.crt
https-trust-hint = ติดตั้งผู้ออกใบรับรองของ Hoard หนึ่งครั้งบนแต่ละอุปกรณ์ แล้วเบราว์เซอร์ในอุปกรณ์นั้นจะเชื่อถือเซิร์ฟเวอร์นี้ (และใบรับรองที่ Hoard สร้างให้ในภายหลัง) ผู้ออกใบรับรองนี้รับรองได้เฉพาะ Hoard นี้เท่านั้น ลายนิ้วมือ (SHA-256):
https-ca-windows = เปิดไฟล์ → Install Certificate → Current User → Place all certificates in the following store → Browse → Trusted Root Certification Authorities → Finish แล้วรีสตาร์ตเบราว์เซอร์
https-ca-macos = เปิดไฟล์ ไฟล์จะเข้าไปอยู่ใน Keychain Access ดับเบิลคลิก “{ $subject }” ในนั้น → Trust → When using this certificate: Always Trust
https-ca-ios-title = iPhone และ iPad
https-ca-ios = ดาวน์โหลดใน Safari แล้วกดอนุญาต Settings → Profile Downloaded → Install จากนั้น Settings → General → About → Certificate Trust Settings → เปิด “{ $subject }”
https-ca-android = Settings → Security → More security settings → Encryption & credentials → Install a certificate → CA certificate แล้วเลือกไฟล์ (ชื่อเมนูอาจต่างกันเล็กน้อยในแต่ละรุ่น ลองค้นหาคำว่า “certificate” ใน Settings)
https-ca-linux-1 = Debian และ Ubuntu:{ " " }
https-ca-linux-2 = { " " }Fedora และ Arch:{ " " }
https-ca-linux-3 = { " " }Chrome บน Linux ใช้รายการของตัวเอง: Settings → Privacy and security → Security → Manage certificates → Import
https-ca-firefox = Firefox ใช้รายการของตัวเอง: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import แล้วเลือก “Trust this CA to identify websites”
https-ca-cli-title = เครื่องมือบรรทัดคำสั่ง hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = { " " }หรือใส่พาธไว้ใน{ " " }
https-ca-cli-after = { "" }
https-options = ตัวเลือก
https-redirect = ส่งเบราว์เซอร์บนอุปกรณ์อื่นจาก http:// ไปยัง https://
https-redirect-hint = ส่งต่อเฉพาะการโหลดหน้าเว็บเท่านั้น เครื่องมือบรรทัดคำสั่ง hoard, ตัวทำงาน Unreal, สคริปต์ และแอป AI ยังใช้งานผ่าน http:// ได้ต่อไป รวมถึง http://localhost บนตัวเซิร์ฟเวอร์เอง
https-http3-hint-before = เร็วขึ้นบน Wi-Fi ที่ไม่เสถียรและบนโทรศัพท์ ต้องใช้ HTTPS และเปิดพอร์ต UDP { $port } นอกเหนือจาก TCP (ใน Docker:{ " " }
https-http3-hint-after = )
https-http3-running = กำลังทำงาน
https-http3-waits = จะเริ่มเมื่อเปิด HTTPS
https-http3-failed = เริ่ม HTTP/3 ไม่ได้: { $error }
https-lockout-title = หากคุณเข้าไม่ได้
https-lockout-1 = { "" }
https-lockout-2 = { " " }บนตัวเซิร์ฟเวอร์เองใช้งานผ่าน http:// ธรรมดาได้เสมอ หรือปิด HTTPS จากบรรทัดคำสั่งของเซิร์ฟเวอร์แล้วเริ่ม Hoard ใหม่:{ " " }
https-lockout-3 = { " " }(ใน Docker:{ " " }
https-lockout-4 = { " " }แล้ว{ " " }
https-lockout-5 = )

## AccountPanel.vue (Settings → Account)

account-role-view = ดูได้
account-role-view-hint = ค้นหา ดูตัวอย่าง และดาวน์โหลด
account-role-edit = แก้ไขได้
account-role-edit-hint = รวมถึงเพิ่ม แก้ไข ติดแท็ก จัดคอลเลกชัน และเก็บถาวร
account-role-admin = ผู้ดูแลระบบ
account-role-admin-hint = รวมถึงการตั้งค่า บัญชี การบำรุงรักษา และการลบถาวร
account-source-internal = รหัสผ่าน Hoard
account-source-ldap = รหัสผ่าน { $directory } ของผู้ใช้
account-source-oauth = ลงชื่อเข้าใช้ด้วย { $provider }
account-source-word-directory = ไดเรกทอรี
account-load-failed = โหลดบัญชีไม่ได้: { $error }
account-failed = { $what }: { $error }
account-password-failed = เปลี่ยนรหัสผ่านไม่ได้
account-password-changed = เปลี่ยนรหัสผ่านแล้ว อุปกรณ์อื่นของคุณถูกลงชื่อออกแล้ว
account-add-failed = เพิ่ม { $name } ไม่ได้
account-added = ตอนนี้ { $name } ลงชื่อเข้าใช้ได้แล้ว
account-role-failed = เปลี่ยนสิทธิ์ของ { $name } ไม่ได้
account-remove-title = นำ { $name } ออกหรือไม่?
account-remove-body = ผู้ใช้จะถูกลงชื่อออกจากทุกที่และลงชื่อเข้าใช้อีกไม่ได้ สิ่งที่ผู้ใช้เพิ่มไว้ในคลังจะไม่ถูกนำออก
account-remove = นำออก
account-remove-failed = นำ { $name } ออกไม่ได้
account-token-failed = สร้างโทเค็นไม่ได้
account-copied = คัดลอกแล้ว
account-copy-token-failed = คัดลอกไม่ได้: เลือกโทเค็นแล้วคัดลอกเอง
account-copy-failed = คัดลอกไม่ได้: เลือกข้อความแล้วคัดลอกเอง
account-revoke-title = เพิกถอน “{ $name }” หรือไม่?
account-revoke-body = สิ่งใดที่ยังใช้โทเค็นนี้อยู่จะถูกปฏิเสธนับจากนี้
account-revoke = เพิกถอน
account-revoke-failed = เพิกถอน “{ $name }” ไม่ได้
account-disconnect-title = ตัดการเชื่อมต่อ { $app } หรือไม่?
account-disconnect-body = แอปจะถูกปฏิเสธนับจากนี้ และต้องเชื่อมต่อใหม่จึงจะใช้ Hoard ได้
account-disconnect = ตัดการเชื่อมต่อ
account-disconnect-failed = ตัดการเชื่อมต่อ { $app } ไม่ได้
account-guest-failed = เปลี่ยนค่านี้ไม่ได้
account-never = ไม่เคย
account-you = คุณ
account-sign-out = ลงชื่อออก
account-signed-in-as-before = ลงชื่อเข้าใช้ในชื่อ{ " " }
account-signed-in-as-admin = { " " }ซึ่งเป็นผู้ดูแลระบบ
account-signed-in-as-edit = { " " }ซึ่งแก้ไขคลังได้
account-signed-in-as-view = { " " }ซึ่งดูได้อย่างเดียวแต่เปลี่ยนแปลงอะไรไม่ได้
account-app-signin = แอป Hoard ลงชื่อเข้าใช้ให้คุณเอง จึงไม่มีรหัสผ่าน สร้างโทเค็นการเข้าถึงด้านล่างสำหรับปลั๊กอินของเอนจินและคำสั่ง hoard
account-password-at-provider = รหัสผ่านของคุณเก็บไว้กับผู้ให้บริการที่คุณใช้ลงชื่อเข้าใช้ จึงต้องเปลี่ยนที่นั่น
account-password-at-directory = รหัสผ่านของคุณเก็บไว้ในไดเรกทอรีที่คุณใช้ลงชื่อเข้าใช้ จึงต้องเปลี่ยนที่นั่น
account-current-password = รหัสผ่านปัจจุบัน
account-new-password = รหัสผ่านใหม่
account-new-password-again = รหัสผ่านใหม่อีกครั้ง
account-change-password = เปลี่ยนรหัสผ่าน
account-password-mismatch = รหัสผ่านใหม่ไม่ตรงกัน
account-password-rule = อย่างน้อย 8 ตัวอักษร
account-people = ผู้ใช้
account-people-hint = แต่ละคนดูได้ (ค้นหา ดูตัวอย่าง และดาวน์โหลด) แก้ไขคลังได้ด้วย (เพิ่ม แก้ไข ติดแท็ก จัดคอลเลกชัน และเก็บถาวร) หรือเป็นผู้ดูแลระบบด้วย (การตั้งค่า บัญชี การบำรุงรักษา และการลบถาวร)
account-you-tag = (คุณ)
account-signs-in-oauth = ลงชื่อเข้าใช้ด้วยผู้ให้บริการ OAuth
account-signs-in-directory = ลงชื่อเข้าใช้ด้วยไดเรกทอรี
account-since = ตั้งแต่ { $date }
account-what-can-do = สิ่งที่ { $name } ทำได้
account-name = ชื่อ
account-how-sign-in = วิธีลงชื่อเข้าใช้
account-password-label = รหัสผ่าน (อย่างน้อย 8 ตัวอักษร)
account-password-placeholder = รหัสผ่าน (8 ตัวอักษรขึ้นไป)
account-what-they-can-do = สิ่งที่ทำได้
account-add = เพิ่ม
account-ai-apps = Claude และแอป AI อื่น ๆ
account-ai-apps-hint = เชื่อมต่อ Claude (หรือแอป MCP ใดก็ได้) แล้วแอปจะค้นหาและดึงแอสเซ็ตให้คุณ รวมถึงอธิบายและติดแท็กสิ่งที่ Hoard ยังไม่ได้ทำ แอปจะลงชื่อเข้าใช้ Hoard ในครั้งแรก โดยคุณอนุญาตที่นี่ในเบราว์เซอร์
account-mcp-address = ที่อยู่ MCP ของ Hoard
account-copy = คัดลอก
account-claude-code-1 = เรียกใช้{ " " }
account-claude-code-2 = { " " }แล้วใช้{ " " }
account-claude-code-3 = { " " }เพื่อลงชื่อเข้าใช้{ " " }
account-claude-code-4 = { " " }จะอธิบายแพ็ก
account-claude-ai = Settings → Connectors → Add custom connector วางที่อยู่ แล้วกด Connect แอปเหล่านี้เชื่อมต่อจากเซิร์ฟเวอร์ของ Anthropic ดังนั้น Hoard ต้องเข้าถึงได้จากอินเทอร์เน็ตผ่าน HTTPS
account-claude-ai-local = Settings → Connectors → Add custom connector วางที่อยู่ แล้วกด Connect แอปเหล่านี้เชื่อมต่อจากเซิร์ฟเวอร์ของ Anthropic ดังนั้น Hoard ต้องเข้าถึงได้จากอินเทอร์เน็ตผ่าน HTTPS (ที่อยู่นี้เข้าถึงไม่ได้: ตั้ง ที่อยู่สาธารณะ ใน การตั้งค่า → การเชื่อมต่อ)
account-connection-meta = สำหรับ { $user } ตั้งแต่ { $since } · ใช้ล่าสุด { $used }
account-no-apps = ยังไม่มีแอปที่เชื่อมต่อ
account-live = กำลังใช้ Hoard อยู่
account-live-hint = แอปที่ขอบางอย่างจาก Hoard ภายใน 10 นาทีที่ผ่านมา รายการนี้อัปเดตเอง
account-live-none = ไม่มีอะไรขออะไรจาก Hoard ภายใน 10 นาทีที่ผ่านมา
account-live-last = ล่าสุด: { $what } · { $when } · คำขอ: { $calls }
account-tokens = โทเค็นการเข้าถึง
account-tokens-hint-1 = สำหรับเครื่องมือบรรทัดคำสั่ง{ " " }
account-tokens-hint-2 = { " " }สคริปต์ ไคลเอนต์ MCP และตัวทำงาน Unreal: ซึ่งส่งโทเค็นในรูปแบบ{ " " }
account-tokens-hint-3 = { " " }โทเค็นทำได้ตามที่คุณเลือก ไม่เกินสิ่งที่คุณทำได้ แต่จัดการบัญชีหรือโทเค็นไม่ได้เลย โทเค็นที่ดูได้อย่างเดียวก็เพียงพอสำหรับการค้นหา ดาวน์โหลด และซิงก์โปรเจกต์
account-tokens-admin = คุณเห็นโทเค็นและแอปที่เชื่อมต่อของทุกคน ส่วนคนอื่นเห็นเฉพาะของตัวเอง
account-made-before = { "" }
account-made-after = : คัดลอกตอนนี้เลย โทเค็นจะไม่แสดงอีก
account-new-token = โทเค็นใหม่
account-cli-1 = สำหรับ CLI:{ " " }
account-cli-2 = { " " }หรือใส่ไว้ใน{ " " }
account-cli-3 = { "" }
account-done = เสร็จ
account-token-meta = สร้างเมื่อ { $made } · ใช้ล่าสุด { $used }
account-token-meta-by = สร้างเมื่อ { $made } โดย { $by } · ใช้ล่าสุด { $used }
account-no-tokens = ยังไม่มีโทเค็น
account-token-name = สิ่งที่จะใช้โทเค็น
account-token-name-placeholder = สิ่งที่จะใช้โทเค็น เช่น “ตัวทำงาน Unreal”
account-token-role = สิ่งที่โทเค็นทำได้
account-make-token = สร้างโทเค็น
account-guests = ผู้เยี่ยมชม
account-guests-allow = ให้ทุกคนที่เข้าถึงเซิร์ฟเวอร์ได้เรียกดูโดยไม่ต้องลงชื่อเข้าใช้
account-guests-hint = ผู้เยี่ยมชมค้นหา ดูตัวอย่าง และดาวน์โหลดได้ แต่เพิ่ม แก้ไข หรือลบอะไรไม่ได้ และไม่เห็นการตั้งค่า

## SignIn.vue

signin-create-title = สร้างบัญชีของคุณ
signin-title = ลงชื่อเข้าใช้
signin-setup-lede = เซิร์ฟเวอร์นี้ยังไม่มีบัญชี บัญชีที่คุณสร้างที่นี่ทำได้ทุกอย่าง เพิ่มบัญชีให้คนอื่นได้ภายหลังจาก การตั้งค่า → บัญชี
signin-overlay-lede = ทุกคนเรียกดูที่นี่ได้ แต่การเปลี่ยนแปลงต้องใช้บัญชี
signin-with = ลงชื่อเข้าใช้ด้วย { $provider }
signin-or-password = หรือใช้ชื่อและรหัสผ่าน
signin-username = ชื่อผู้ใช้
signin-name = ชื่อ
signin-new-password = รหัสผ่านใหม่
signin-password = รหัสผ่าน
signin-password-rule = อย่างน้อย 8 ตัวอักษร
signin-confirm-password = ยืนยันรหัสผ่าน
signin-mismatch = รหัสผ่านทั้งสองไม่ตรงกัน
signin-setup-code = รหัสตั้งค่า
signin-setup-code-hint-before = รหัสที่กำหนดให้เซิร์ฟเวอร์นี้ใน{ " " }
signin-setup-code-hint-after = { "" }
signin-creating = กำลังสร้าง…
signin-signing-in = กำลังลงชื่อเข้าใช้…
signin-create = สร้างบัญชี
signin-submit = ลงชื่อเข้าใช้
signin-keep-browsing = เรียกดูต่อ
signin-directory-hint = ใช้ชื่อและรหัสผ่าน { $directory } ของคุณ
signin-forgot-1 = ลืมรหัสผ่าน Hoard หรือ? บนเซิร์ฟเวอร์ ให้เรียกใช้{ " " }
signin-forgot-2 = { " " }(ใน Docker ใช้{ " " }
signin-forgot-3 = ) เพื่อตั้งรหัสผ่านใหม่

## SetupWizard.vue (first visit to a new server)

setup-title = ควรเก็บคลังของคุณไว้ที่ใด?
setup-lede = เลือกโฟลเดอร์บนเซิร์ฟเวอร์ให้ Hoard เก็บแพ็กของคุณ หรือชี้ไปยังโฟลเดอร์แพ็กที่คุณมีอยู่แล้ว คุณเพิ่มตำแหน่งอื่นบนดิสก์อื่นได้ภายหลังจากการตั้งค่า
setup-lede-desktop = เลือกโฟลเดอร์ให้ Hoard เก็บแพ็กของคุณ หรือชี้ไปยังโฟลเดอร์แพ็กที่คุณมีอยู่แล้ว คุณเพิ่มตำแหน่งอื่นบนดิสก์อื่นได้ภายหลังจากการตั้งค่า
setup-unreachable = ติดต่อเซิร์ฟเวอร์ไม่ได้: { $error }
setup-unreachable-desktop = Hoard ไม่ตอบสนอง: { $error }
setup-try-again = ลองใหม่
setup-loading = กำลังโหลด…
setup-start = เริ่มคลัง
setup-ready = คลังของคุณพร้อมแล้ว
setup-managed-before = แพ็กใหม่จะเข้าไปที่{ " " }
setup-managed-after = { " " }วางไฟล์ โฟลเดอร์ หรือไฟล์บีบอัดที่ใดก็ได้บนหน้านี้เพื่อเพิ่มแพ็กแรกของคุณ
setup-watched-before = Hoard กำลังเฝ้าดู{ " " }
setup-watched-after = { " " }แพ็กจะปรากฏเมื่อพบ หากต้องการอัปโหลดด้วย ให้เพิ่มคลังที่ Hoard ดูแลจาก การตั้งค่า → ตำแหน่งคลัง
setup-open = เปิดคลัง

## ConnectApp.vue (an app such as Claude asks to sign in)

connect-title = เชื่อมต่อ { $app } หรือไม่?
connect-lede-1 = { " " }จะค้นหา ดาวน์โหลด อธิบาย และติดแท็กทุกอย่างใน Hoard นี้ได้ โดยทำในนามของ{ " " }
connect-lede-2 = { " " }แต่จัดการบัญชีหรือโทเค็นการเข้าถึงไม่ได้
connect-back-before = คุณจะกลับไปที่{ " " }
connect-back-after = { " " }ตัดการเชื่อมต่อได้ทุกเมื่อที่ การตั้งค่า → บัญชี
connect-allow = อนุญาต
connect-cancel = ยกเลิก
connect-failed = เชื่อมต่อแอปนี้ไม่ได้
connect-go-library = ไปที่คลัง
connect-checking = กำลังตรวจสอบ…

## InsecureNotice.vue (the line shown over plain http://)

insecure-text = นี่คือ http:// ธรรมดา บางอย่างจึงต้องใช้ HTTPS: 3 มิติด้วย WebGPU, คลิปบอร์ดสมัยใหม่, การติดตั้ง Hoard เป็นแอป และ Claude connectors
insecure-switch = สลับไปที่ https://
insecure-set-up = ตั้งค่า HTTPS
insecure-dismiss = ไม่ต้องแสดงอีก

## FormatWizard.vue (Settings → Store files → teaching Hoard a store's file)

formatwizard-too-big = ไฟล์นี้ใหญ่เกินกว่าจะอ่านเป็นไฟล์ร้านค้า
formatwizard-not-json = ไฟล์นี้ไม่ใช่ JSON ที่ถูกต้อง จึงแสดงเป็นข้อความ
formatwizard-read-failed = อ่านไฟล์ไม่ได้: { $error }
formatwizard-back-title = กลับไปที่ฟอร์มหรือไม่?
formatwizard-back-body = การเปลี่ยนแปลงใน TOML จะหายไป: ฟอร์มจะเขียนรูปแบบใหม่จากข้อมูลที่มี
formatwizard-back-to-form = กลับไปที่ฟอร์ม
formatwizard-miss-name = ไฟล์ของรูปแบบนี้ไม่รับชื่อ { $file } เพิ่มชื่อนี้ใน ไฟล์
formatwizard-miss-not-the-stores = สิ่งที่ใช้จดจำไฟล์ของร้านค้าไม่อยู่ในไฟล์นี้ เปลี่ยน จดจำจาก
formatwizard-miss-no-title = ไม่พบชื่อในไฟล์นี้ เลือกชื่อ
formatwizard-miss-short-name = ไม่ใช่ชื่อที่รับ
formatwizard-miss-short-not-the-stores = ไม่ใช่ของร้านค้านี้
formatwizard-miss-short-no-title = ไม่มีชื่อ
formatwizard-saved-packs = บันทึก { $file } แล้ว กำลังอ่านแพ็ก { $packs } แพ็กอีกครั้ง
formatwizard-saved = บันทึก { $file } แล้ว
formatwizard-save-failed = บันทึกรูปแบบไม่ได้: { $error }
formatwizard-zip-failed = สร้างไฟล์ zip ไม่ได้: { $error }
formatwizard-stop-title = หยุดสอนไฟล์นี้หรือไม่?
formatwizard-stop-body = รูปแบบจะไม่ถูกบันทึก
formatwizard-stop = หยุด
formatwizard-label = ตัวช่วยสร้างรูปแบบ
formatwizard-changing = กำลังแก้ไข { $name }
formatwizard-starting-from = เริ่มจาก { $name }
formatwizard-title = สอน Hoard ให้รู้จักไฟล์ของร้านค้า
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }ใน{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = แก้ไขเป็น TOML
formatwizard-file-reference = ไฟล์สำหรับอ้างอิง
formatwizard-click-json = คลิก{ $field }ในไฟล์
formatwizard-click-text = คลิก{ $field }ในไฟล์ หรือเลือกข้อความนั้น
formatwizard-pick-first = เลือกฟิลด์ทางขวา แล้วคลิกค่าของฟิลด์ที่นี่
formatwizard-loading = กำลังโหลด…
formatwizard-the-format = รูปแบบ
formatwizard-syntax-before = ไวยากรณ์อยู่ใน{ " " }
formatwizard-syntax-after = { "" }
formatwizard-store = ร้านค้า
formatwizard-store-placeholder = ชื่อร้านค้า เช่น Kenney
formatwizard-files = ไฟล์
formatwizard-recognize = จดจำจาก
formatwizard-recognize-json = คีย์ที่มีเฉพาะในไฟล์ของร้านค้า
formatwizard-recognize-text = บรรทัดที่มีเฉพาะในไฟล์ของร้านค้า
formatwizard-recognize-hint = บรรทัดละหนึ่งวิธี ตรงกับวิธีใดก็ได้ จำเป็นเมื่อไฟล์อื่นมีชื่อเดียวกัน
formatwizard-seller = ผู้ขาย
formatwizard-seller-placeholder = เมื่อไฟล์ไม่ได้ระบุ
formatwizard-tags = แท็ก
formatwizard-tags-placeholder = สำหรับทุกสิ่งที่ไฟล์อธิบาย คั่นด้วยจุลภาค
formatwizard-fields = ฟิลด์
formatwizard-selector = ตัวเลือก { $field }
formatwizard-clear = ล้าง { $field }
formatwizard-pick = เลือก
formatwizard-what-it-reads = สิ่งที่อ่านได้
formatwizard-read-title = ชื่อ
formatwizard-read-seller = ผู้ขาย
formatwizard-read-license = สัญญาอนุญาต
formatwizard-read-url = หน้าร้านค้า
formatwizard-read-kind = ประเภท
formatwizard-read-tags = แท็ก
formatwizard-read-origin = สร้างใน
formatwizard-read-size = ขนาด
formatwizard-also = { $format } อ่านไฟล์นี้ได้เช่นกัน รูปแบบนี้จะถูกลองก่อน
formatwizard-library = ไฟล์ { $files } ไฟล์ใน { $packs } แพ็กมีชื่อที่รูปแบบนี้รับ จาก { $tried } ไฟล์ที่ลอง อ่านได้ { $read } ไฟล์
formatwizard-library-none = ไม่มีไฟล์ในคลังที่มีชื่อที่รูปแบบนี้รับ
formatwizard-share = แบ่งปัน
formatwizard-share-hint = ไฟล์ zip ของรูปแบบนี้ โดยมีไฟล์นี้เป็นตัวอย่างและผลการอ่านตัวอย่าง จัดวางแบบเดียวกับรีโพสิทอรีของ Hoard เพื่อส่งเข้าไปให้ทุกคนได้ใช้ นำสิ่งที่ไม่ควรแบ่งปันออกจากตัวอย่างก่อน
formatwizard-sample = ตัวอย่างที่จะแบ่งปัน
formatwizard-download = ดาวน์โหลด { $file }
formatwizard-close = ปิด
formatwizard-use = ใช้ที่นี่
formatwizard-cancel = ยกเลิก
settings-unreal-editor-what = ติดตั้ง Unreal Editor สำหรับกระบวนการส่งออก เรียกใช้กระบวนการแยกต่างหากเพื่อส่งออกแอสเซ็ต
settings-unreal-download = ดาวน์โหลดเอดิเตอร์ Linux x86-64 ของ Epic หนึ่งครั้งลงในโฟลเดอร์เครื่องมือของ Hoard ใช้พื้นที่ดิสก์หลายสิบ GB และต้องมีบัญชี GitHub ที่ Epic อนุญาตแล้ว
settings-unreal-github-step-link-go = วิธีเชื่อมโยง
settings-unreal-github-step-token = สร้างโทเค็น GitHub (classic) โดยเลือกเฉพาะ read:packages
settings-unreal-github-step-token-go = สร้างโทเค็น
settings-unreal-github-user = ชื่อผู้ใช้ GitHub
settings-unreal-github-token = โทเค็น
settings-unreal-github-token-kept = บันทึกแล้ว (พิมพ์โทเค็นใหม่เพื่อแทนที่)
settings-unreal-github-save = บันทึกการลงชื่อเข้าใช้
settings-unreal-github-forget = ยกเลิกการเชื่อมต่อ
settings-unreal-github-saved = บันทึกการลงชื่อเข้าใช้ GitHub แล้ว กำลังลองดาวน์โหลดอีกครั้ง
settings-unreal-github-forgotten = ยกเลิกการเชื่อมต่อ GitHub แล้ว
settings-unreal-github-from-server = เซิร์ฟเวอร์นี้ได้รับการลงชื่อเข้าใช้ของตัวเองด้วย (GHCR_TOKEN_FILE หรือ DOCKER_CONFIG) ซึ่งจะใช้เมื่อไม่มีการบันทึกไว้ที่นี่
settings-unreal-github-connect = เชื่อมต่อ GitHub…
settings-unreal-github-connect-title = เชื่อมต่อ GitHub
settings-unreal-github-connect-why = Epic แชร์ Unreal Editor ให้บัญชี GitHub ที่ลิงก์กับ Epic Games Hoard ขอแค่สิทธิ์อ่านแพ็กเกจเท่านั้น
settings-unreal-github-sign-in = ลงชื่อเข้าใช้ด้วย GitHub
settings-unreal-github-enter-code = เมื่อคลิก GitHub จะเปิดในแท็บใหม่ ป้อนรหัสนี้ที่นั่น
settings-unreal-github-waiting = กำลังรอ GitHub…
settings-unreal-github-expired = รหัสนั้นหมดอายุแล้ว
settings-unreal-github-denied = การลงชื่อเข้าใช้ถูกยกเลิกที่ GitHub
settings-unreal-github-failed = ไม่ได้รับรหัสจาก GitHub
settings-unreal-github-again = ขอรหัสใหม่
settings-unreal-github-before = ก่อนเริ่ม
settings-unreal-github-before-what = ลิงก์ GitHub กับบัญชี Epic Games ของคุณ และยอมรับคำเชิญ EpicGames ที่ GitHub ส่งทางอีเมล
settings-unreal-github-use-token = ใช้โทเค็นแทน
settings-unreal-github-use-sign-in = ลงชื่อเข้าใช้ด้วย GitHub แทน
settings-unreal-github-no-password = ไม่มีรหัสผ่านใดไปถึง Hoard
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = เชื่อมต่อเป็น { $user } แล้ว กำลังลองดาวน์โหลดอีกครั้ง
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = เชื่อมต่อเป็น { $user }
settings-unreal-github-as-what = Unreal Editor ดาวน์โหลดในนามบัญชี GitHub นี้
settings-unreal-github-none = ยังไม่ได้เชื่อมต่อ GitHub
settings-unreal-github-none-what = จำเป็นสำหรับดาวน์โหลดเอดิเตอร์
settings-unreal-github-change = เปลี่ยน…
settings-unreal-image = เวอร์ชันหรืออิมเมจ Unreal
settings-editor-path = ป้อนเส้นทางไฟล์ปฏิบัติการของ { $tool } ที่ติดตั้งไว้ การติดตั้งอัตโนมัติใช้ได้เฉพาะใน Docker
error-unreal-image = ป้อนเวอร์ชัน Unreal 5 แท็กเวอร์ชัน Epic dev/dev-slim หรือการอ้างอิงอิมเมจ Epic แบบ SHA-256
error-github-no-app = เซิร์ฟเวอร์นี้ไม่มีแอป GitHub สำหรับลงชื่อเข้าใช้ ใช้โทเค็นแทน
error-github-unreachable = เชื่อมต่อ GitHub ไม่ได้
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub ปฏิเสธการลงชื่อเข้าใช้: { $reason }

-- ===================================================
-- Test data for SysUser table (simplified, using default timestamps)
-- ===================================================

-- 1. 创建表（如果已经通过 migration 创建，可以注释掉）
-- CREATE TABLE sys_user (
--     id VARCHAR PRIMARY KEY,
--     name VARCHAR NOT NULL,
--     gender VARCHAR NOT NULL,
--     account VARCHAR NOT NULL,
--     password VARCHAR NOT NULL,
--     mobile_phone VARCHAR NOT NULL,
--     birthday DATE NOT NULL,
--     enabled BOOLEAN NOT NULL,
--     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );
-- 2. 创建触发器函数，自动更新时间
CREATE OR REPLACE FUNCTION update_updated_at()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 3. 创建触发器
DROP TRIGGER IF EXISTS trigger_update_sys_user ON sys_user;

CREATE TRIGGER trigger_update_sys_user
    BEFORE UPDATE
    ON sys_user
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
-- 2. 插入测试数据（省略 created_at 和 updated_at，使用默认值）
INSERT INTO sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled)
VALUES ('1', 'Alice', 'Female', 'alice001', '123456', '13800000001', '1990-01-01', TRUE),
       ('2', 'Bob', 'Male', 'bob002', '123456', '13800000002', '1988-05-12', TRUE),
       ('3', 'Carol', 'Female', 'carol003', '123456', '13800000003', '1992-07-23', TRUE),
       ('4', 'David', 'Male', 'david004', '123456', '13800000004', '1985-11-11', TRUE),
       ('5', 'Eva', 'Female', 'eva005', '123456', '13800000005', '1995-03-15', TRUE),
       ('6', 'Frank', 'Male', 'frank006', '123456', '13800000006', '1987-08-08', TRUE),
       ('7', 'Grace', 'Female', 'grace007', '123456', '13800000007', '1991-12-21', TRUE),
       ('8', 'Henry', 'Male', 'henry008', '123456', '13800000008', '1989-02-28', TRUE),
       ('9', 'Ivy', 'Female', 'ivy009', '123456', '13800000009', '1993-06-06', TRUE),
       ('10', 'Jack', 'Male', 'jack010', '123456', '13800000010', '1990-09-09', TRUE),
       ('11', 'Kate', 'Female', 'kate011', '123456', '13800000011', '1992-04-04', TRUE),
       ('12', 'Leo', 'Male', 'leo012', '123456', '13800000012', '1986-10-10', TRUE),
       ('13', 'Mia', 'Female', 'mia013', '123456', '13800000013', '1994-01-30', TRUE),
       ('14', 'Nick', 'Male', 'nick014', '123456', '13800000014', '1988-07-07', TRUE),
       ('15', 'Olivia', 'Female', 'olivia015', '123456', '13800000015', '1991-03-03', TRUE),
       ('16', 'Paul', 'Male', 'paul016', '123456', '13800000016', '1989-11-11', TRUE),
       ('17', 'Quinn', 'Female', 'quinn017', '123456', '13800000017', '1993-05-05', TRUE),
       ('18', 'Ryan', 'Male', 'ryan018', '123456', '13800000018', '1990-08-08', TRUE),
       ('19', 'Sophia', 'Female', 'sophia019', '123456', '13800000019', '1992-12-12', TRUE),
       ('20', 'Tom', 'Male', 'tom020', '123456', '13800000020', '1987-06-06', TRUE),
       ('21', 'Uma', 'Female', 'uma021', '123456', '13800000021', '1995-09-09', TRUE);


-- 3. 查询测试
-- 查询所有用户
SELECT *
FROM sys_user;

-- 查询指定性别的用户
SELECT *
FROM sys_user
WHERE gender = 'F';

-- 查询指定账号的用户
SELECT *
FROM sys_user
WHERE account = 'alice001';

-- 查询启用的用户
SELECT *
FROM sys_user
WHERE enabled = TRUE;

-- 分页查询（前 5 条）
SELECT *
FROM sys_user
ORDER BY created_at
LIMIT 5;

-- 聚合查询
-- 统计男女数量
SELECT gender, COUNT(*) AS count
FROM sys_user
GROUP BY gender;

-- 模糊查询
SELECT *
FROM sys_user
WHERE name LIKE 'A%';

-- 排序查询
SELECT *
FROM sys_user
ORDER BY birthday DESC;

-- 更新数据
UPDATE sys_user
SET mobile_phone = '13888888888'
WHERE account = 'bob002';

-- 删除数据
DELETE
FROM sys_user
WHERE account = 'tom020';

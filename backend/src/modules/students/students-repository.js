const { processDBRequest } = require("../../utils");

/** Match the seeded role by name instead of hardcoding a role id. */
const STUDENT_ROLE_NAME = "Student";

const findAllStudents = async (payload) => {
    const { name, className, section, roll } = payload;
    let query = `
        SELECT
            t1.id,
            t1.name,
            t1.email,
            t1.last_login AS "lastLogin",
            t1.is_active AS "systemAccess"
        FROM users t1
        INNER JOIN roles role ON role.id = t1.role_id AND role.name ILIKE $1
        LEFT JOIN user_profiles t3 ON t1.id = t3.user_id
        WHERE 1=1`;
    let queryParams = [STUDENT_ROLE_NAME];
    if (name) {
        query += ` AND t1.name = $${queryParams.length + 1}`;
        queryParams.push(name);
    }
    if (className) {
        query += ` AND t3.class_name = $${queryParams.length + 1}`;
        queryParams.push(className);
    }
    if (section) {
        query += ` AND t3.section_name = $${queryParams.length + 1}`;
        queryParams.push(section);
    }
    if (roll) {
        query += ` AND t3.roll = $${queryParams.length + 1}`;
        queryParams.push(roll);
    }

    query += " ORDER BY t1.id";

    const { rows } = await processDBRequest({ query, queryParams });
    return rows;
};

const addOrUpdateStudent = async (payload) => {
    const query = "SELECT * FROM student_add_update($1)";
    const queryParams = [payload];
    const { rows } = await processDBRequest({ query, queryParams });
    return rows[0];
};

const findStudentDetail = async (id) => {
    const query = `
        SELECT
            u.id,
            u.name,
            u.email,
            u.is_active AS "systemAccess",
            p.phone,
            p.gender,
            -- Cast the date columns to text so they stay calendar dates instead
            -- of being shifted into UTC timestamps by the driver.
            p.dob::text AS dob,
            p.class_name AS "class",
            p.section_name AS "section",
            p.roll,
            p.father_name AS "fatherName",
            p.father_phone AS "fatherPhone",
            p.mother_name AS "motherName",
            p.mother_phone AS "motherPhone",
            p.guardian_name AS "guardianName",
            p.guardian_phone AS "guardianPhone",
            p.relation_of_guardian as "relationOfGuardian",
            p.current_address AS "currentAddress",
            p.permanent_address AS "permanentAddress",
            p.admission_dt::text AS "admissionDate",
            reporter.name as "reporterName"
        FROM users u
        INNER JOIN roles role ON role.id = u.role_id AND role.name ILIKE $2
        LEFT JOIN user_profiles p ON u.id = p.user_id
        LEFT JOIN users reporter ON u.reporter_id = reporter.id
        WHERE u.id = $1`;
    const queryParams = [id, STUDENT_ROLE_NAME];
    const { rows } = await processDBRequest({ query, queryParams });
    return rows[0];
};

const findStudentToSetStatus = async ({ userId, reviewerId, status }) => {
    const now = new Date();
    const query = `
        UPDATE users u
        SET
            is_active = $1,
            status_last_reviewed_dt = $2,
            status_last_reviewer_id = $3
        FROM roles role
        WHERE u.id = $4
          AND role.id = u.role_id
          AND role.name ILIKE $5
    `;
    const queryParams = [status, now, reviewerId, userId, STUDENT_ROLE_NAME];
    const { rowCount } = await processDBRequest({ query, queryParams });
    return rowCount;
};

module.exports = {
    findAllStudents,
    addOrUpdateStudent,
    findStudentDetail,
    findStudentToSetStatus,
};

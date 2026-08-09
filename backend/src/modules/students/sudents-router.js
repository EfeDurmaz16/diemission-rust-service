const express = require("express");
const router = express.Router();
const { checkApiAccess } = require("../../middlewares");
const studentController = require("./students-controller");

router.get("", checkApiAccess, studentController.handleGetAllStudents);
router.post("", checkApiAccess, studentController.handleAddStudent);
router.get("/:id", checkApiAccess, studentController.handleGetStudentDetail);
router.post("/:id/status", checkApiAccess, studentController.handleStudentStatus);
router.put("/:id", checkApiAccess, studentController.handleUpdateStudent);

module.exports = { studentsRoutes: router };

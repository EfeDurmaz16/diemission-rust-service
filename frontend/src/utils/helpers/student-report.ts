const REPORT_SERVICE_URL = import.meta.env.VITE_REPORT_SERVICE_URL || 'http://localhost:8080';

/**
 * PDF report endpoint on the Rust microservice. Used as a plain link target so
 * the browser follows the Content-Disposition header and downloads the file.
 */
export const getStudentReportUrl = (studentId: number | string): string =>
  `${REPORT_SERVICE_URL}/api/v1/students/${studentId}/report`;

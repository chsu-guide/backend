#![allow(dead_code)]
use constcat::concat;
pub const BASE_URL: &str = "http://api.chsu.ru/api";
const AUTH_VALID: &str = "/auth/valid";
const AUTH_SIGNIN: &str = "/auth/signin";
const BUILDING: &str = "/building/v1";
const STUDENT_GROUP: &str = "/group/v2";
const DEPARTMENT: &str = "/department/v2";
const AUDITORIUM: &str = "/auditorium/v1";
const TIMETABLE: &str = "/timetable/v1";
const DISCIPLINE: &str = "/discipline/v1";
const TEACHERS: &str = "/teacher/v1";

pub const AUTH_VALID_URL: &str = concat!(BASE_URL, AUTH_VALID);
pub const AUTH_SIGNIN_URL: &str = concat!(BASE_URL, AUTH_SIGNIN);
pub const BUILDING_URL: &str = concat!(BASE_URL, BUILDING);
pub const STUDENT_GROUP_URL: &str = concat!(BASE_URL, STUDENT_GROUP);
pub const DEPARTMENT_URL: &str = concat!(BASE_URL, DEPARTMENT);
pub const AUDITORIUM_URL: &str = concat!(BASE_URL, AUDITORIUM);
pub const TIMETABLE_URL: &str = concat!(BASE_URL, TIMETABLE);
pub const DISCIPLINE_URL: &str = concat!(BASE_URL, DISCIPLINE);
pub const TEACHERS_URL: &str = concat!(BASE_URL, TEACHERS);

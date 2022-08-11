include "../base.thrift"
include "enums.thrift"
include "course.thrift"
include "room.thrift"

namespace go ek.ecp.after_school

enum ScheduleRuleType {
    Soft = 0
    Hard = 1
}

enum LoopMode {
    Null = 0
    Week = 1
    Month = 2
}

enum SearchType {
    LessonID = 1
    CourseID = 2

}

enum ScheduleConflictType {
    BanTime = 1 
    SameTimeDuplicatedClass = 2 
    SameTimeDuplicatedTeacher = 3 
    SameTimeDuplicatedRoom = 4 
}

enum LessonStatus {
    NotStarted = 0
    InProgress = 1
    Finished = 2
}

struct ScheduleRuleBanTime {
    1: i64 start_time
    2: i64 end_time
}

struct ScheduleRule {
    1: i64 rule_id
    2: string name
    3: ScheduleRuleType type
    10: optional list<ScheduleRuleBanTime> ban_time_list
    50: i64 creator_id
    51: i64 created_at
    52: i64 updater_id
    53: i64 updated_at
}

struct LessonTime{
    1: i64 serial_num
    2: i64 start_time 
    3: i64 end_time 
}

struct Lesson {
    1: i64 id
    2: i64 serial_num 
    3: i64 class_id
    4: string class_name
    5: i64 teacher_id
    6: string teacher_name
    7: i64 room_id
    8: string room_name
    9: LessonTime lesson_time
    10: LessonStatus status
}

struct LessonClass {
    1: i64 teacher_id
    2: i64 room_id
    3: i64 class_id
}

struct LessonPlan {
    1: i64 start_date 
    2: LoopMode loop_mode
    3: list<LessonTime> lesson_times 
}

struct CourseSchedule {
    1: i64 course_schedule_id
    2: string name
    3: i64 term_id
    4: i64 course_id
    5: list<enums.Grade> grades
    6: i64 start_date
    7: LoopMode loop_mode
    8: list<LessonPlan> lesson_plans
    9: list<LessonClass> lesson_classes
    10: i64 student_capacity
    50: i64 creator_id
    51: i64 created_at
    52: i64 updater_id
    53: i64 updated_at
}

struct ClassSchedule {
    1: i64 id
    2: i64 term_id
    3: i64 class_id
    4: string class_naame
    5: list<Lesson> lessons
    50: i64 creator_id
    51: i64 created_at
    52: i64 updater_id
    53: i64 updated_at
}

struct TeacherSchedule {
    1: i64 id
    2: i64 term_id
    3: i64 teacher_id
    4: i64 teacher_name
    5: list<Lesson> lessons
    50: i64 creator_id
    51: i64 created_at
    52: i64 updater_id
    53: i64 updated_at
}

struct StudentSchedule {
    1: i64 id
    2: i64 term_id
    3: i64 student_id
    4: i64 student_name
    5: list<Lesson> lessons
    50: i64 creator_id
    51: i64 created_at
    52: i64 updater_id
    53: i64 updated_at
}

struct RoomSchedule {
    1: i64 id
    2: i64 term_id
    3: i64 room_id
    4: i64 room_name
    5: list<Lesson> lessons
    50: i64 creator_id
    51: i64 created_at
    52: i64 updater_id
    53: i64 updated_at
}

struct CourseChoice {
    1: i64 id
    2: i64 term_id
    3: i64 student_id
    4: i64 course_schedule_id
    5: i64 class_id
    50: i64 creator_id
    51: i64 created_at
    52: i64 updater_id
    53: i64 updated_at
}

struct ScheduleConflict {
    1: LessonTime lesson_time
    2: ScheduleConflictType type
    3: string hint
}

struct CourseChoiceDetail {
    1: i64 term_id
    2: i64 student_id
    3: string student_name
    4: i64 student_grade
    5: i64 course_schedule_id
    6: i64 class_id
    7: i64 class_name
    8: list<LessonTime> lesson_times
    9: i64 choose_time
    10: i64 admin_class_id
    11: i64 admin_class_name
    12: i64 admin_teacher_id
    13: i64 admin_teacher_name
}

struct ClassDetail {
    1: i64 class_id
    2: i64 class_name
    3: i64 class_student_cnt
    4: list<LessonTime> lesson_times
    5: list<room.Room> rooms;
    7: i64 administrative_class_id
    8: i64 administrative_class_name
    9: i64 virtual_class_teacher_id
    10: i64 virtual_class_teacher_name
}

struct ScheduleTeacher {
    1: i64 teacher_id
    2: i64 teacher_name
    3: bool available
}

struct ScheduleRoom {
    1: i64 room_id
    2: i64 room_name
    3: bool available
}



struct AddCourseScheduleRulesReq {
    1: i64 term_id
    2: list<ScheduleRule> rules

    30: i64 org_id 
    31: i64 job_key 
    32: i64 user_id 
    255: optional base.Base Base
}

struct AddCourseScheduleRulesResp {
    255: required base.BaseResp BaseResp
}

struct DeleteCourseScheduleRulesReq {
    1: i64 term_id
    2: list<i64> rule_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct DeleteCourseScheduleRulesResp {
    255: required base.BaseResp BaseResp
}

struct GetCourseScheduleRuleReq {
    1: i64 term_id

    30: i64 root_org_id; 
    31: i64 org_id; 
    32: i64 job_key; 
    33: i64 user_id; 
    255: optional base.Base Base
}

struct GetCourseScheduleRuleResp {
    1: list<ScheduleRule> rules
    255: required base.BaseResp BaseResp
}

struct UpdateCourseScheduleRuleReq {
    1: i64 term_id
    2: i64 rule_id
    3: optional string name
    10: optional list<ScheduleRuleBanTime> ban_time_list

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct UpdateCourseScheduleRuleResp {
    255: required base.BaseResp BaseResp
}

struct AddCourseScheduleReq {
    1: i64 term_id
    2: CourseSchedule course_schedule

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct AddCourseScheduleResp {
    1: bool has_conflict
    2: list<ScheduleConflict> schedule_conflicts
    255: required base.BaseResp BaseResp
}

struct DeleteCourseScheduleReq {
    1: i64 term_id
    2:i64 course_schedule_id

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct DeleteCourseScheduleResp {
    255: required base.BaseResp BaseResp
}

struct GetCourseSchedulesReq {
    1: i64 term_id
    10: optional list<i64> course_schedule_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetCourseSchedulesResp {
    1: list<CourseSchedule> course_schedules
    255: required base.BaseResp BaseResp
}

struct CheckCourseScheduleConflictReq {
    1: CourseSchedule course_schedule

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct CheckCourseScheduleConflictResp {
    1: bool has_conflict
    2: list<ScheduleConflict> schedule_conflicts
    255: required base.BaseResp BaseResp
}

struct UpdateCourseScheduleReq {
    1: CourseSchedule course_schedule

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct UpdateCourseScheduleResp {
    1: bool has_conflict
    2: list<ScheduleConflict> schedule_conflicts
    255: required base.BaseResp BaseResp
}

struct GetScheduleTeachersReq {
    1: i64 term_id
    2: LoopMode loop_mode
    3: i64 start_time 
    4: i64 end_time 

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetScheduleTeachersResp {
    1: list<ScheduleTeacher> teachers
    255: required base.BaseResp BaseResp
}

struct GetScheduleRoomsReq {
    1: i64 term_id
    2: LoopMode loop_mode
    3: i64 start_time 
    4: i64 end_time 

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetScheduleRoomsResp {
    1: list<ScheduleRoom> rooms
    255: required base.BaseResp BaseResp
}

struct GetClassSchedulesReq {
    1: i64 term_id
    2: list<i64> class_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetClassSchedulesResp {
    1: list<ClassSchedule> class_schedules
    255: required base.BaseResp BaseResp
}

struct CheckClassScheduleConflictReq {
    1: CourseSchedule course_schedule
    255: optional base.Base Base
}

struct CheckClassScheduleConflictResp {
    1: bool has_conflict
    2: list<ScheduleConflict> schedule_conflicts
    255: required base.BaseResp BaseResp
}

struct UpdateClassScheduleReq {
    1: i64 term_id
    2: i64 class_id
    10: optional list<Lesson> lessons

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct UpdateClassScheduleResp {
    1: bool has_conflict
    2: list<ScheduleConflict> schedule_conflicts
    255: required base.BaseResp BaseResp
}

struct GetTeacherSchedulesReq {
    1: i64 term_id
    2: list<i64> teacher_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetTeacherSchedulesResp {
    1: list<TeacherSchedule> teacher_schedules
    255: required base.BaseResp BaseResp
}

struct GetStudentSchedulesReq {
    1: i64 term_id
    2: list<i64> student_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetStudentSchedulesResp {
    1: list<StudentSchedule> student_schedules
    255: required base.BaseResp BaseResp
}

struct GetRoomSchedulesReq {
    1: i64 term_id
    2: list<i64> room_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetRoomSchedulesResp {
    1: list<RoomSchedule> room_schedules
    255: required base.BaseResp BaseResp
}

struct MakeCourseChoiceReq {
    1: i64 term_id
    2: i64 student_id
    3: i64 course_schedule_id

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct MakeCourseChoiceResp {
    255: required base.BaseResp BaseResp
}

struct CancelCourseChoiceReq {
    1: i64 term_id
    2: i64 student_id
    3: i64 course_schedule_id

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct CancelCourseChoiceResp {
    255: required base.BaseResp BaseResp
}

struct GetCourseChoicesReq {
    1: i64 term_id
    2: i64 course_id (go.tag='validate:"required"'); 
    10: optional list<i64> student_ids
    11: optional list<i64> class_ids
    12: optional list<enums.Grade> grades
    20: required i32 offset,
    21: required i32 limit,

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetCourseChoicesResp {
    1: i32 total,
    2: optional list<CourseChoiceDetail> course_choices
    255: required base.BaseResp BaseResp
}

struct GetClassesReq {
    1: optional list<i64> class_ids
    2: optional list<i64> room_ids
    3: optional list<i64> teacher_ids
    4: optional list<enums.ClassStatus> class_statuses

    20: required i32 offset,
    21: required i32 limit,

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetClassesResp {
    1: i64 total
    2: optional list<ClassDetail> classes
    255: required base.BaseResp BaseResp
}

struct SearchClassesCond {
    1: optional string mix_name;
    2: optional enums.ClassStatus class_status;
    3: optional i64 course_id;
}

struct SearchClassesReq {
    1: optional SearchClassesCond cond;

    20: required i32 offset,
    21: required i32 limit,

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct SearchClassesResp {
    1: i64 total
    2: optional list<ClassDetail> classes
    255: required base.BaseResp BaseResp
}

struct UpdateClassReq {
    1: i64 class_id
    10: optional string class_name

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct UpdateClassResp {
    255: required base.BaseResp BaseResp
}

struct GetClassStudentsReq {
    1: i64 term_id
    2: required i64 class_id
    3: optional string mix 

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct ClassStudentDetail {
    1: i64 student_id
    2: string student_name
    3: i64 user_id
    4: string user_name
    5: i64 admin_class_id
    6: i64 admin_class_name
}

struct GetClassStudentsResp {
    1: list<ClassStudentDetail> class_students
    255: required base.BaseResp BaseResp
}

struct ModifyClassStudentsReq {
    1: i64 term_id
    2: i64 from_class_id
    3: i64 to_class_id
    4: list<i64> student_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct ModifyClassStudentsResp {
    255: required base.BaseResp BaseResp
}

struct AddCourseScheduleTimeReq {
    1: required i64 term_id
    2: required i64 start_time
    3: required i64 end_time

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct AddCourseScheduleTimeResp {
    255: required base.BaseResp BaseResp
}

struct UpdateCourseScheduleTimeReq {
    1: required i64 term_id
    2: required i64 start_time
    3: required i64 end_time

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct UpdateCourseScheduleTimeResp {
    255: required base.BaseResp BaseResp
}

struct DeleteCourseScheduleTimeReq {
    1: required i64 term_id

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct DeleteCourseScheduleTimeResp {
    255: required base.BaseResp BaseResp
}

struct GetCourseScheduleTimeReq {
    1: required i64 term_id

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetCourseScheduleTimeResp {
    1: required i64 start_time
    2: required i64 end_time
    255: required base.BaseResp BaseResp
}

struct AddCourseChoiceTimeReq {
    1: required i64 term_id
    2: required i64 start_time
    3: required i64 end_time

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct AddCourseChoiceTimeResp {
    255: required base.BaseResp BaseResp
}

struct UpdateCourseChoiceTimeReq {
    1: required i64 term_id
    2: required i64 start_time
    3: required i64 end_time

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct UpdateCourseChoiceTimeResp {
    255: required base.BaseResp BaseResp
}

struct DeleteCourseChoiceTimeReq {
    1: required i64 term_id

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct DeleteCourseChoiceTimeResp {
    255: required base.BaseResp BaseResp
}

struct GetCourseChoiceTimeReq {
    1: required i64 term_id

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetCourseChoiceTimeResp {
    1: required i64 start_time
    2: required i64 end_time
    255: required base.BaseResp BaseResp
}

struct GetLessonsReq {
    1: optional list<i64> class_ids
    2: optional list<i64> teacher_ids
    3: optional list<i64> room_ids
    4: optional list<i64> lesson_ids

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key(api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')
    255: optional base.Base Base
}

struct GetLessonsResp {
    1: list<Lesson> lessons
    255: required base.BaseResp BaseResp
}

struct CourseWithSchedule {
    1: course.Course course;
    2: CourseSchedule course_schedule;
}

struct SearchOpenCourseCond {
    1: optional string mix_name; 
    2: optional i64 course_category_id; 
    4: optional enums.AgencyType template_declare_org_type; 
    5: optional list<course.CourseChoiceStatus> return_course_choice_status_total_list; 
}

struct SearchOpenCourseReq {
    1: optional SearchOpenCourseCond cond;
    20: required i32 offset(go.tag='validate:"min=0"'),    
    21: required i32 limit(go.tag='validate:"min=0,max=255"'),     

    30: optional i64 root_org_id (api.header = 'X-ek-root_org_id')
    31: optional i64 org_id (api.header = 'X-ek-org_id')
    32: optional i64 job_key (api.header = 'X-ek-job_key') 
    33: optional i64 user_id (api.header = 'X-ek-user_id')

    255: optional base.Base Base;
}

struct SearchOpenCourseResp {
    1: list<CourseWithSchedule> courses;
    2: i32 total; 
    3: map<course.CourseChoiceStatus, i32> course_choice_status_total_map; 

    255: base.BaseResp BaseResp;
}
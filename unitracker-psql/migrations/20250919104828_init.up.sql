-- Add up migration script here
create table discipline
(
    id    bigint not null
        constraint discipline_pk
            primary key,
    name text   not null
);

comment on column discipline.id is 'Discipline ID';

comment on column discipline.name is 'Discipline name';

alter table discipline
    owner to unitracker;

create table faculty
(
    id    bigint not null
        constraint faculty_pk
            primary key,
    name text   not null
);

comment on column faculty.id is 'Faculty ID';

comment on column faculty.name is 'Full faculty name';

alter table faculty
    owner to unitracker;

create table chair
(
    id    bigint not null
        constraint chair_pk
            primary key,
    name text   not null
);

comment on column chair.id is 'Chair ID';

comment on column chair.name is 'Chair name';

alter table chair
    owner to unitracker;

create table student_group
(
    id         bigint   not null
        constraint groups_pk
            primary key,
    name text     not null,
    course     smallint not null,
    chair_id   bigint
        constraint chair_id
            references chair,
    faculty_id bigint
        constraint faculty_id
            references faculty
);

comment on column student_group.id is 'Group ID';

comment on column student_group.name is 'Group name';

comment on column student_group.course is 'Study year of the group';

alter table student_group
    owner to unitracker;

create table teacher
(
    id          bigint not null
        constraint teacher_pk
            primary key,
    last_name   text   not null,
    first_name  text   not null,
    middle_name text
);

comment on column teacher.id is 'Teacher ID';

comment on column teacher.last_name is 'Surname';

comment on column teacher.first_name is 'First name';

comment on column teacher.middle_name is 'Paternal name, sometimes null';

alter table teacher
    owner to unitracker;

create table building
(
    id    bigint not null
        constraint building_pk
            primary key,
    name text   not null
);

comment on column building.id is 'Building ID';

comment on column building.name is 'Building name';

alter table building
    owner to unitracker;

create table auditorium
(
    id          bigint not null
        constraint auditorium_pk
            primary key,
    name        text   not null,
    number      text   not null,
    building_id bigint
        constraint auditorium_building_fk
            references building
);

comment on column auditorium.id is 'Auditorium ID';

comment on column auditorium.name is 'Auditorium name';

comment on column auditorium.number is 'Auditorium number, floor number + auditorium index + literas';

alter table auditorium
    owner to unitracker;

create table schedule
(
    id               bigint not null
        constraint schedule_pk
            primary key,
    request_date     timestamp not null,
    start_time       timestamp not null,
    end_time         timestamp not null,
    lesson_type      text   not null,
    lesson_type_abbr text,
    discipline_id    bigint not null
        constraint schedule_discipline_fk
            references discipline
);

comment on column schedule.id is 'Class ID';

comment on column schedule.start_time is 'Start time of the class';

comment on column schedule.end_time is 'End time of the class';

comment on column schedule.lesson_type is 'Type of the lesson';

comment on column schedule.lesson_type_abbr is 'Short form of lesson_type';

alter table schedule
    owner to unitracker;

create table schedule_group
(
    schedule_id bigint not null
        constraint schedule_fk
            references schedule,
    group_id    bigint not null
        constraint group_fk
            references student_group,
    constraint schedule_group_pk
        primary key (schedule_id, group_id)
);

alter table schedule_group
    owner to unitracker;

create table schedule_auditorium
(
    schedule_id   bigint not null
        constraint schedule_fk
            references schedule,
    auditorium_id bigint not null
        constraint auditorium_fk
            references auditorium,
    constraint schedule_auditorium_pk
        primary key (auditorium_id, schedule_id)
);

alter table schedule_auditorium
    owner to unitracker;

create table schedule_teacher
(
    schedule_id bigint not null
        constraint schedule_fk
            references schedule,
    teacher_id  bigint not null
        constraint teacher_fk
            references teacher,
    constraint schedule_teacher_pk
        primary key (schedule_id, teacher_id)
);

alter table schedule_teacher
    owner to unitracker;

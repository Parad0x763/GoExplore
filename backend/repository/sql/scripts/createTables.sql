create table Venue (
	venueId uuid primary key default gen_random_uuid(),
	venueName varchar(150) not null,
	location varchar(150) not null,
	socialMedia varchar(250)
);

create table Band (
	bandId uuid primary key default gen_random_uuid(),
	bandName varchar(100) not null unique,
	socialMedia varchar(250)
);

create table Person (
	personId uuid primary key default gen_random_uuid(),
	firstName varchar(50) not null,
	middleName varchar(50) not null,
	lastName varchar(50) not null
);

create table MusicEvent (
	eventId uuid primary key default gen_random_uuid(),
	startDate date not null,
	endDate date not null,
	eventName varchar(250) not null,
	socialMedia varchar(250)
);

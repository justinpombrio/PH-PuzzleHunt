import psycopg2
from sys import exit

try:
    conn = psycopg2.connect("dbname='crums' user='postgres' host='localhost' password='password'")
except:
    print "Can't connect to database!"
    exit(1)

if raw_input("Do you really want to wipe the database? (yes/[no]) ") != "yes":
        exit(0)
if raw_input("Are you sure? (yes/[no]) ") != "yes":
        exit(0)

c = conn.cursor()

string = """
drop table if exists Team;
create table Team (
  teamID serial primary key NOT NULL,
  password varchar NOT NULL,
  name varchar NOT NULL,
  guesses int NOT NULL
);

drop table if exists Member;
create table Member (
  teamID smallint NOT NULL,
  name varchar NOT NULL,
  email varchar NOT NULL
);

drop table if exists Puzzle;
create table Puzzle (
  puzzleID serial primary key NOT NULL,
  name varchar NOT NULL,
  waveID smallint NOT NULL,
  points int NOT NULL,
  number varchar NOT NULL
);

drop table if exists Hint;
create table Hint (
  puzzleID smallint NOT NULL,
  number smallint NOT NULL,
  waveID smallint NOT NULL,
  penalty int NOT NULL,
  key varchar NOT NULL
);

drop table if exists Wave;
create table Wave (
  waveID serial primary key NOT NULL,
  name varchar,
  time timestamp NOT NULL,
  guesses int NOT NULL
);

drop table if exists Guess;
create table Guess (
  teamID smallint NOT NULL,
  puzzleID smallint NOT NULL,
  guess varchar NOT NULL,
  time timestamp NOT NULL
);

drop table if exists Solve;
create table Solve (
  teamID smallint NOT NULL,
  puzzleID smallint NOT NULL,
  time timestamp NOT NULL,
  primary key (teamID, puzzleID)
);

drop table if exists Stats;
create table Stats (
  teamID smallint NOT NULL,
  puzzleID smallint NOT NULL,
  score int NOT NULL,
  solveTime real,
  guesses int NOT NULL
);"""

c.execute(string)
conn.commit()
c.close()
conn.close()

print "RESET"

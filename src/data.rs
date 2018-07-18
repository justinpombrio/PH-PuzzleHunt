use chrono::{Utc, DateTime};
use mustache::{MapBuilder, VecBuilder, Data};
use postgres::rows::Row;


pub trait DBTable {
    fn from_row(row: Row) -> Self;
    fn to_data(&self, builder: MapBuilder) -> MapBuilder;
    fn drop_query() -> &'static str;
    fn init_query() -> &'static str;
    fn test_init_query() -> &'static str;
    fn name() -> &'static str;
    fn names() -> &'static str;
}


pub fn build_data(items: Vec<&AddToData>) -> Data {
    let mut builder = MapBuilder::new();
    for item in &items {
        builder = item.add_to_data(builder);
    }
    builder.build()
}


pub trait AddToData {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder;
}

impl<C : DBTable> AddToData for C {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder.insert_map(Self::name(), |m| self.to_data(m))
    }
}

impl<C : DBTable> AddToData for Vec<C> {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder.insert_vec(C::names(), |b| vec_to_data(self, b))
    }
}

fn vec_to_data<C : DBTable>(items: &Vec<C>, builder: VecBuilder) -> VecBuilder {
    let mut builder = builder;
    for item in items {
        builder = builder.push_map(|map| item.to_data(map))
    }
    builder
}


////// Site //////

#[derive(Debug)]
pub struct Site {
    pub owner: String,
    pub secret: String
}

impl DBTable for Site {
    fn name()  -> &'static str { "site" }
    fn names() -> &'static str { "sites" }

    fn from_row(row: Row) -> Site {
        Site{
            owner:  row.get(0),
            secret: row.get(1)
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("owner",  self.owner.clone())
            .insert_str("secret", self.secret.clone())
    }

    fn drop_query() -> &'static str {
        "drop table if exists Site;"
    }

    fn init_query() -> &'static str {
"create table Site (
  owner varchar NOT NULL,
  secret varchar NOT NULL
);"
    }

    fn test_init_query() -> &'static str {
"insert into Site (owner, secret)
values ('me', 'secret');"
    }
}



////// Hunts //////

#[derive(Debug)]
pub struct Hunt {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub team_size: i32,
    pub init_guesses: i32,
    pub password: String,
    pub closed: bool,
    pub visible: bool
}

impl DBTable for Hunt {
    fn name()  -> &'static str { "hunt" }
    fn names() -> &'static str { "hunts" }
    
    fn from_row(row: Row) -> Hunt {
        Hunt{
            id:           row.get(0),
            name:         row.get(1),
            key:          row.get(2),
            team_size:    row.get(3),
            init_guesses: row.get(4),
            password:     row.get(5),
            closed:       row.get(6),
            visible:      row.get(7)
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("id",          format!("{}", self.id))
            .insert_str("name",        self.name.clone())
            .insert_str("key",         self.key.clone())
            .insert_str("teamSize",    format!("{}", self.team_size))
            .insert_str("initGuesses", format!("{}", self.init_guesses))
            .insert_bool("closed",     self.closed)
            .insert_bool("visible",    self.visible)
    }

    fn drop_query() -> &'static str {
        "drop table if exists Hunt;"
    }

    fn init_query() -> &'static str {
"create table Hunt (
  huntID serial primary key NOT NULL,
  name varchar NOT NULL,
  key varchar NOT NULL,
  teamSize int NOT NULL,
  initGuesses int NOT NULL,
  password varchar NOT NULL,
  closed boolean NOT NULL,
  visible boolean NOT NULL
);"
    }

    fn test_init_query() -> &'static str {
"insert into Hunt (name, key, teamSize, initGuesses, password, closed, visible)
values ('Best Hunt Ever', 'besthuntever', 4, 100, 'pass', true, true);"
    }
}


////// Waves //////


#[derive(Debug)]
pub struct Wave {
    pub name: String,
    pub hunt: i32,
    pub time: DateTime<Utc>,
    pub guesses: i32,
    pub released: bool,
    pub puzzles: Vec<Puzzle>
}

impl DBTable for Wave {
    fn name()  -> &'static str { "wave" }
    fn names() -> &'static str { "waves" }
    
    fn from_row(row: Row) -> Wave {
        Wave{
            name:     row.get(0),
            hunt:     row.get(1),
            time:     row.get(2),
            guesses:  row.get(3),
            released: row.get(4),
            puzzles:  vec!()
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("name",      self.name.clone())
            .insert_str("hunt",      format!("{}", self.hunt))
            .insert_str("time",      format!("{}", self.time))
            .insert_str("guesses",   format!("{}", self.guesses))
            .insert_bool("released", self.released)
            .insert_vec("puzzles",   |b| vec_to_data(&self.puzzles, b))
    }

    fn drop_query() -> &'static str {
        "drop table if exists Wave;"
    }

    fn init_query() -> &'static str {
"create table Wave (
  name varchar NOT NULL,
  hunt int NOT NULL,
  time timestamp with time zone NOT NULL,
  guesses int NOT NULL,
  released boolean NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Wave (name, hunt, time, guesses, released)
values ('Wave One', 1, '2004-10-19 10:23:54', 10, true);"
    }
}


////// Puzzles //////

#[derive(Debug)]
pub struct Puzzle {
    pub name: String,
    pub number: String,
    pub hunt: i32,
    pub base_points: i32,
    pub current_points: i32,
    pub answer: String,
    pub wave: String,
    pub key: String,
    pub released: bool,
    pub hints: Vec<Hint>
}

impl DBTable for Puzzle {
    fn name()  -> &'static str { "puzzle" }
    fn names() -> &'static str { "puzzles" }
    
    fn from_row(row: Row) -> Puzzle {
        Puzzle{
            name:           row.get(0),
            number:         row.get(1),
            hunt:           row.get(2),
            base_points:    row.get(3),
            current_points: row.get(4),
            answer:         row.get(5),
            wave:           row.get(6),
            key:            row.get(7),
            released:       row.get(8),
            hints:          vec!()
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("name",          self.name.clone())
            .insert_str("number",        self.number.clone())
            .insert_str("hunt",          format!("{}", self.hunt))
            .insert_str("basePoints",    format!("{}", self.base_points))
            .insert_str("currentPoints", format!("{}", self.current_points))
            .insert_str("wave",          self.wave.clone())
            .insert_str("key",           self.key.clone())
            .insert_bool("released",     self.released)
            .insert_vec("hints",         |b| vec_to_data(&self.hints, b))
    }

    fn drop_query() -> &'static str {
        "drop table if exists Puzzle;"
    }

    fn init_query() -> &'static str {
"create table Puzzle (
  name varchar primary key NOT NULL,
  number varchar NOT NULL,
  hunt int NOT NULL,
  basePoints int NOT NULL,
  currentPoints int NOT NULL,
  answer varchar NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL,
  released boolean NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Puzzle (name, number, hunt, basePoints, currentPoints, answer, wave, key, released)
values ('Puzzle One', '#1', 1, 2, 1, 'answer1', 'Wave One', 'PPP', true),
       ('Puzzle Two', '#2', 1, 3, 2, 'answer2', 'Wave One', 'QQQ', true),
       ('Puzzle Three', '#3', 1, 3, 2, 'answer3', 'Wave One', 'RRR', false);"
    }
}


////// Hints //////

#[derive(Debug)]
pub struct Hint {
    pub puzzle: String,
    pub number: i32,
    pub hunt: i32,
    pub penalty: i32,
    pub wave: String,
    pub key: String,
    pub released: bool
}

impl DBTable for Hint {
    fn name()  -> &'static str { "hint" }
    fn names() -> &'static str { "hints" }
    
    fn from_row(row: Row) -> Hint {
        Hint{
            puzzle:   row.get(0),
            number:   row.get(1),
            hunt:     row.get(2),
            penalty:  row.get(3),
            wave:     row.get(4),
            key:      row.get(5),
            released: row.get(6)
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("puzzle",    self.puzzle.clone())
            .insert_str("number",    format!("{}", self.number))
            .insert_str("hunt",      format!("{}", self.hunt))
            .insert_str("penalty",   format!("{}", self.penalty))
            .insert_str("wave",      self.wave.clone())
            .insert_str("key",       self.key.clone())
            .insert_bool("released", self.released)
    }

    fn drop_query() -> &'static str {
        "drop table if exists Hint;"
    }

    fn init_query() -> &'static str {
"create table Hint (
  puzzle varchar NOT NULL,
  number int NOT NULL,
  hunt int NOT NULL,
  penalty int NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL,
  released boolean NOT NULL
);
"        
    }

    fn test_init_query() -> &'static str {
"insert into Hint (puzzle, number, hunt, penalty, wave, key, released)
values ('Puzzle One', 1, 1, 1, 'Wave One', 'HHH', true);"
    }
}


////// Teams //////

#[derive(Debug)]
pub struct Team {
    pub team_id: i32,
    pub hunt: i32,
    pub password: String,
    pub name: String,
    pub guesses: i32,
    pub members: Vec<Member>
}

impl DBTable for Team {
    fn name()  -> &'static str { "team" }
    fn names() -> &'static str { "teams" }
    
    fn from_row(row: Row) -> Team {
        Team{
            team_id:  row.get(0),
            hunt:     row.get(1),
            password: row.get(2),
            name:     row.get(3),
            guesses:  row.get(4),
            members:  vec!()
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("teamID",   format!("{}", self.team_id))
            .insert_str("hunt",     format!("{}", self.hunt))
            .insert_str("password", self.password.clone())
            .insert_str("name",     self.name.clone())
            .insert_str("guesses",  format!("{}", self.guesses))
            .insert_vec("members",  |b| vec_to_data(&self.members, b))
    }
    
    fn drop_query() -> &'static str {
        "drop table if exists Team;"
    }

    fn init_query() -> &'static str {
"create table Team (
  teamID serial primary key NOT NULL,
  hunt int NOT NULL,
  password varchar NOT NULL,
  name varchar NOT NULL,
  guesses int NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Team (hunt, password, name, guesses)
values (1, 'pass', 'BestTeamEver', 50);"
    }
}


////// Members //////

#[derive(Debug)]
pub struct Member {
    pub team_id: i32,
    pub hunt: i32,
    pub name: String,
    pub email: String
}

impl DBTable for Member {
    fn name()  -> &'static str { "member" }
    fn names() -> &'static str { "members" }
    
    fn from_row(row: Row) -> Member {
        Member{
            team_id: row.get(0),
            hunt:    row.get(1),
            name:    row.get(2),
            email:   row.get(3)
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("teamID", format!("{}", self.team_id))
            .insert_str("hunt", format!("{}", self.hunt))
            .insert_str("name", self.name.clone())
            .insert_str("email", self.email.clone())
    }

    fn drop_query() -> &'static str {
        "drop table if exists Member;"
    }

    fn init_query() -> &'static str {
"create table Member (
  teamID int NOT NULL,
  hunt int NOT NULL,
  name varchar NOT NULL,
  email varchar NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Member (teamID, hunt, name, email)
values (1, 1, 'BestPersonEver', 'person@email.com');"
    }
}


////// Guesses //////

#[derive(Debug)]
pub struct Guess {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle: String,
    pub guess: String,
    pub time: DateTime<Utc>
}

impl DBTable for Guess {
    fn name()  -> &'static str { "guess" }
    fn names() -> &'static str { "guesss" }
    
    fn from_row(row: Row) -> Guess {
        Guess{
            team_id: row.get(0),
            hunt:    row.get(1),
            puzzle:  row.get(2),
            guess:   row.get(3),
            time:    row.get(4)
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("teamID", format!("{}", self.team_id))
            .insert_str("hunt", format!("{}", self.hunt))
            .insert_str("puzzle", self.puzzle.clone())
            .insert_str("guess", self.guess.clone())
            .insert_str("time", format!("{}", self.time))
    }

    fn drop_query() -> &'static str {
        "drop table if exists Guess;"
    }

    fn init_query() -> &'static str {
"create table Guess (
  teamID int NOT NULL,
  hunt int NOT NULL,
  puzzle varchar NOT NULL,
  guess varchar NOT NULL,
  time timestamp with time zone NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Guess (teamID, hunt, puzzle, guess, time)
values (1, 1, 'Puzzle One', 'answer?', '2004-10-19 10:23:54');"
    }
}


////// Solves //////

pub struct Solve {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle: String,
    pub time: DateTime<Utc>
}

impl DBTable for Solve {
    fn name()  -> &'static str { "solve" }
    fn names() -> &'static str { "solves" }
    
    fn from_row(row: Row) -> Solve {
        Solve{
            team_id: row.get(0),
            hunt:    row.get(1),
            puzzle:  row.get(2),
            time:    row.get(3)
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("teamID", format!("{}", self.team_id))
            .insert_str("hunt",   format!("{}", self.hunt))
            .insert_str("puzzle", self.puzzle.clone())
            .insert_str("time",   format!("{}", self.time))
    }
    
    fn drop_query() -> &'static str {
        "drop table if exists Solve;"
    }

    fn init_query() -> &'static str {
"create table Solve (
  teamID int NOT NULL,
  hunt int NOT NULL,
  puzzle varchar NOT NULL,
  time timestamp with time zone NOT NULL,
  primary key (teamID, puzzle)
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Solve (teamID, hunt, puzzle, time)
values (1, 1, 'Puzzle One', '2004-10-19 10:23:54');"
    }
}


////// Stats //////

#[derive(Debug)]
pub struct Stat {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle: String,
    pub score: i32,
    pub solve_time: i32,
    pub guesses: i32
}

impl DBTable for Stat {
    fn name()  -> &'static str { "stat" }
    fn names() -> &'static str { "stats" }
    
    fn from_row(row: Row) -> Stat {
        Stat{
            team_id:    row.get(0),
            hunt:       row.get(1),
            puzzle:     row.get(2),
            score:      row.get(3),
            solve_time: row.get(4),
            guesses:    row.get(5)
        }
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("teamID",    format!("{}", self.team_id))
            .insert_str("hunt",      format!("{}", self.hunt))
            .insert_str("puzzle",    format!("{}", self.puzzle))
            .insert_str("score",     format!("{}", self.score))
            .insert_str("solveTime", format!("{}", self.solve_time))
            .insert_str("guesses",   format!("{}", self.guesses))
    }

    fn drop_query() -> &'static str {
        "drop table if exists Stats;"
    }

    fn init_query() -> &'static str {
"create table Stats (
  teamID int NOT NULL,
  hunt int NOT NULL,
  puzzle varchar NOT NULL,
  score int NOT NULL,
  solveTime int,
  guesses int NOT NULL,
  primary key (teamID, puzzle)
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Stats (teamId, hunt, puzzle, score, solveTime, guesses)
values (1, 1, 'Puzzle One', 10, 385, 50);"
    }
}

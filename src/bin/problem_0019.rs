fn main() {
    dbg!(WeekDay::from(Date::new(1900, 1, 1)));
    dbg!(WeekDay::from(Date::new(1971, 7, 3)));
    dbg!(solve());
}

fn solve() -> usize {
    let mut sum = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            if let WeekDay::Sunday = WeekDay::from(Date::new(year, month, 1)) {
                sum += 1;
            }
        }
    }
    sum
}

#[derive(Debug)]
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

struct Date {
    year: usize,
    month: usize,
    day: usize,
}

impl Date {
    fn new(year: usize, month: usize, day: usize) -> Self {
        Self {
            year: year,
            month: month,
            day: day,
        }
    }
}

impl From<Date> for WeekDay {
    fn from(date: Date) -> Self {
        //count how many days have passed since 1.1.0001
        let mut days_prev_years = (date.year - 1) * 365;
        //leap years
        let prev_leap_years = (date.year - 1) / 4 - (date.year - 1) / 100 + (date.year - 1) / 400;
        days_prev_years += prev_leap_years;
        //is the current year a leap year?
        let is_leap_year = date.year % 4 == 0 && date.year % 100 != 0 || date.year % 400 == 0;
        //days since this year started
        let days_per_month: [usize; 12] = [
            31,
            if is_leap_year { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        let days_this_year = days_per_month.iter().take(date.month - 1).sum::<usize>();
        //total days since 1.1.0001
        let total_days = days_prev_years + days_this_year + date.day;
        match (total_days + 6) % 7 {
            0 => WeekDay::Monday,
            1 => WeekDay::Tuesday,
            2 => WeekDay::Wednesday,
            3 => WeekDay::Thursday,
            4 => WeekDay::Friday,
            5 => WeekDay::Saturday,
            6 => WeekDay::Sunday,
            _ => unreachable!(),
        }
    }
}

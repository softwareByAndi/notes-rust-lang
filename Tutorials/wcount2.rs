// wcount2.rs: This version reads one line at a time.
// Illustrates lifetime issue (for s: remove String::from in line 15).

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let mut word_count = HashMap::new();

    let file = File::open("gettysburg.txt").expect("file open error");
    for line in io::BufReader::new(file).lines() {
        if let Ok(s) = line {
            for word in s.split_whitespace() {
                *word_count.entry(String::from(word)).or_insert(0) += 1;
            }
        }
    }

    for (w, c) in &word_count {
        println!("{}: {}", w, c);
    }
}

/* Output:
new: 2
cause: 1
nation,: 3
the: 9
or: 2
here,: 2
shall: 3
equal.: 1
long: 2
dedicate: 2
add: 1
proper: 1
seven: 1
brave: 1
dead: 2
but: 1
field,: 1
nation: 2
fought: 1
resolve: 1
poor: 1
living: 1
is: 3
rather: 1
portion: 1
who: 3
dead,: 1
be: 2
it,: 1
little: 1
note,: 1
fitting: 1
all: 1
far: 2
continent,: 1
those: 1
on: 2
our: 2
forget: 1
come: 1
can: 5
--: 7
struggled: 1
lives: 1
The: 2
honored: 1
for: 5
dedicated,: 1
met: 1
final: 1
war.: 1
resting: 1
unfinished: 1
great: 3
whether: 1
should: 1
take: 1
measure: 1
a: 7
this.: 1
that: 13
endure.: 1
they: 3
testing: 1
Now: 1
which: 2
devotion: 2
freedom: 1
live.: 1
here: 5
God,: 1
sense,: 1
engaged: 1
civil: 1
perish: 1
as: 1
so: 3
detract.: 1
are: 3
power: 1
vain: 1
might: 1
say: 1
remaining: 1
above: 1
We: 2
war,: 1
work: 1
any: 1
not: 5
under: 1
us: 3
last: 1
world: 1
conceived: 2
ground.: 1
larger: 1
battle-field: 1
government: 1
and: 6
we: 8
nor: 1
people,: 3
this: 3
forth: 1
living,: 1
did: 1
score: 1
remember: 1
years: 1
ago: 1
here.: 1
by: 1
dedicated: 3
in: 4
thus: 1
what: 2
It: 3
full: 1
before: 1
brought: 1
to: 8
these: 2
proposition: 1
never: 1
increased: 1
died: 1
men,: 1
earth.: 1
consecrated: 1
Liberty,: 1
of: 5
nobly: 1
consecrate: 1
task: 1
created: 1
hallow: 1
advanced.: 1
rather,: 1
fathers: 1
place: 1
gave: 2
from: 2
will: 1
birth: 1
highly: 1
men: 1
have: 5
But,: 1
it: 1
Four: 1
their: 1
altogether: 1
do: 1
*/

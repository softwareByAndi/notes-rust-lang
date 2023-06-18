// wcount.rs: The quintessential word count program.

use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let text = read_to_string("gettysburg.txt").unwrap();
    let mut word_count = HashMap::new();

    // or_insert returns a mutable reference! Use * to dereference.
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    for (w, c) in &word_count {
        println!("{}: {}", w, c);
    }
    println!("");

    // Sort by key
    let mut v: Vec<_> = word_count.into_iter().collect();
    v.sort();
    for (k, v) in v {
        println!("{}: {}", k, v);
    }
}

/* Output:
did: 1
proposition: 1
so: 3
living: 1
advanced.: 1
before: 1
who: 3
Liberty,: 1
here: 5
say: 1
honored: 1
But,: 1
in: 4
final: 1
can: 5
last: 1
remember: 1
The: 2
our: 2
increased: 1
dedicate: 2
from: 2
rather,: 1
dead: 2
seven: 1
great: 3
met: 1
nobly: 1
cause: 1
God,: 1
freedom: 1
the: 9
remaining: 1
created: 1
nation: 2
war.: 1
under: 1
are: 3
here,: 2
thus: 1
measure: 1
they: 3
birth: 1
endure.: 1
their: 1
a: 7
forget: 1
do: 1
living,: 1
perish: 1
it,: 1
men,: 1
come: 1
poor: 1
devotion: 2
long: 2
far: 2
equal.: 1
We: 2
all: 1
continent,: 1
larger: 1
score: 1
ground.: 1
unfinished: 1
government: 1
what: 2
or: 2
battle-field: 1
detract.: 1
should: 1
conceived: 2
work: 1
as: 1
dedicated: 3
of: 5
years: 1
have: 5
live.: 1
shall: 3
above: 1
will: 1
men: 1
never: 1
place: 1
us: 3
this: 3
resting: 1
nation,: 3
--: 7
these: 2
engaged: 1
on: 2
rather: 1
new: 2
nor: 1
fought: 1
forth: 1
take: 1
this.: 1
we: 8
full: 1
fathers: 1
died: 1
proper: 1
hallow: 1
people,: 3
little: 1
portion: 1
field,: 1
brought: 1
might: 1
It: 3
world: 1
that: 13
note,: 1
be: 2
lives: 1
those: 1
whether: 1
and: 6
Four: 1
sense,: 1
dead,: 1
consecrated: 1
Now: 1
is: 3
dedicated,: 1
it: 1
which: 2
task: 1
resolve: 1
any: 1
vain: 1
by: 1
not: 5
civil: 1
altogether: 1
highly: 1
here.: 1
ago: 1
power: 1
but: 1
earth.: 1
to: 8
add: 1
brave: 1
gave: 2
consecrate: 1
fitting: 1
for: 5
struggled: 1
testing: 1
war,: 1
*/

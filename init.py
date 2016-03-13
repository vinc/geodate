import sys
import ephem
from datetime import timezone

def write_vec(f, name):
    f.write('    pub static ref %sS: Vec<i64> = {\n' % name.upper())
    f.write('        vec![\n')
    date = ephem.date('1970-01-01') # Unix Epoch
    while date < ephem.date('2033-05-18'): # 2 gigaseconds later
        date = getattr(ephem, 'next_%s' % name)(date)
        time = int(date.datetime().replace(tzinfo=timezone.utc).timestamp())
        f.write('            %i,\n' % time) # FIXME: Remove last comma
    f.write('        ]\n')
    f.write('    };\n')

with open('src/data.rs', 'w') as f:
    f.write('lazy_static! {\n')
    write_vec(f, 'solstice')
    write_vec(f, 'new_moon')
    f.write('}\n')

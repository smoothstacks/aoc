run year day:
    #!/bin/bash
    day=`printf %02d {{day}}`
    cargo r -p aoc{{year}}day${day}
test year day:
    #!/bin/bash
    day=`printf %02d {{day}}`
    cargo t -p aoc{{year}}day${day}
create year day:
    #!/bin/bash
    day=`printf %02d {{day}}`

    mkdir -p years/{{year}}/${day}
    cd years/{{year}}/${day};  cargo generate --path ../../../.template --name aoc{{year}}day${day} --init;

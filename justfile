run year day *extra:
    #!/bin/bash
    day=`printf %02d {{day}}`
    cargo r -p aoc{{year}}day${day} {{extra}}
test year day *extra:
    #!/bin/bash
    day=`printf %02d {{day}}`
    cargo t -p aoc{{year}}day${day} {{extra}}
bench year day:
    #!/bin/bash
    day=`printf %02d {{day}}`
    cargo bench -p aoc{{year}}day${day}
flamegraph year day *extra:
    #!/bin/bash
    day=`printf %02d {{day}}`
    cargo flamegraph -b aoc{{year}}day${day} {{extra}}
bacon year day:
    #!/bin/bash
    day=`printf %02d {{day}}`
    bacon --project years/{{year}}/${day}
time year day *extra:
    #!/bin/bash
    time just run {{year}} {{day}} {{extra}};
create year day:
    #!/bin/bash
    day=`printf %02d {{day}}`

    mkdir -p years/{{year}}/${day}
    cd years/{{year}}/${day};
    cargo generate --path ../../../.template --name aoc{{year}}day${day} --init;
    cargo r -p aoc-util --bin getinput --release {{year}} {{day}} --out input.txt

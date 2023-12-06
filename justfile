run day:
    cargo r -p {{day}}
test day:
    cargo t -p {{day}}
create day:
    cd days;  cargo generate --path ../.template --name {{day}};

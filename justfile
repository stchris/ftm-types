download-sample-data:
    mkdir -p sample/
    wget --continue --show-progress -O sample/de_abgeordnetenwatch_sidejobs.ftm.json https://data.ftm.store/de_abgeordnetenwatch_sidejobs/entities.ftm.json
    wget --continue --show-progress --verbose -O sample/de_abgeordnetenwatch_sponsoring.ftm.json https://data.ftm.store/de_abgeordnetenwatch_sponsoring/entities.ftm.json

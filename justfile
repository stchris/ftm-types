default: 
    just --list

download-sample-data:
    mkdir -p sample/
    wget --continue --show-progress -O sample/de_abgeordnetenwatch_sidejobs.ftm.json https://data.ftm.store/de_abgeordnetenwatch_sidejobs/entities.ftm.json
    wget --continue --show-progress --verbose -O sample/de_abgeordnetenwatch_sponsoring.ftm.json https://data.ftm.store/de_abgeordnetenwatch_sponsoring/entities.ftm.json

download-ftm-schema VERSION:
    mkdir -p schemas/{{VERSION}}/
    wget https://github.com/opensanctions/followthemoney/archive/refs/tags/v{{VERSION}}.zip
    unzip v{{VERSION}}.zip >/dev/null
    mv followthemoney-{{VERSION}}/followthemoney/schema/*.yaml schemas/{{VERSION}}/
    rm -rf followthemoney-{{VERSION}}/ v{{VERSION}}.zip

test:
  cargo test --features builder

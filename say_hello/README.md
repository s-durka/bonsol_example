
# say_hello

This is a Bonsol zkprogram, built on risc0

bonsol build --zk-program-path .
bonsol deploy url --url http://localhost:8080 --manifest-path ./manifest.json
bonsol prove -m manifest.json -e "zzzzz" -i inputs.json
bonsol execute -f execution_request.json --wait
cargo build --release

scp target/armv5te-unknown-linux-musleabi/release/ev3-Schoolproject robot@$1:/home/robot/Schoolproject/executable

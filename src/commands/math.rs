// Copyright (c) 2018 FrictionlessPortals and the frictionlessbot contributors
// See the README.md file at the top-level directory of this distribution.
// This project is licensed under the license listed in the github project.

// Multiply two numbers
// # Example #
// ~multiply 2 2
// => 4
command!(multiply(_ctx, msg, args) {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let product = one * two;

    if let Err(why) = msg.channel_id.send_message(|m| m
        .embed(|e| e
        .title(product)
        .description("Output from the multiplication command."))){
        error!("Error sending message: {:?}", why);
    }
});

// Add two numbers
// # Example #
// ~add 2 2
// => 4
command!(addition(_ctx, msg, args) {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let sum = one * two;

    if let Err(why) = msg.channel_id.send_message(|m| m
        .embed(|e| e
        .title(sum)
        .description("Output from the addition command."))){
        error!("Error sending message: {:?}", why);
    }
});

// Subtract two numbers
// # Example #
// ~subtract 2 2
// => 0
command!(subtract(_ctx, msg, args) {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let difference = one - two;

    if let Err(why) = msg.channel_id.send_message(|m| m
        .embed(|e| e
        .title(difference)
        .description("Output from the subtraction command."))){
        error!("Error sending message: {:?}", why);
    }
});

// Divide two numbers
// # Example #
// ~divide 2 2
// => 1
command!(divide(_ctx, msg, args) {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let division = one / two;

    if let Err(why) = msg.channel_id.send_message(|m| m
        .embed(|e| e
        .title(division)
        .description("Output from the division command."))){
        error!("Error sending message: {:?}", why);
    }
});

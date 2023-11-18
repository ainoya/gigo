# Gigo

gigo means "garbage in, garbage out". It is a command line interface (CLI) tool that uses the LLM to generate text based on your input.

Currently it only supports OpenAI's API, but I plan to add support for other LLMs in the future.

This is a simple command line interface (CLI) tool written in Rust. It takes standard input, sends it to the OpenAI API, and outputs the result to the standard output.

## Prerequisites

- An OpenAI API key

## Setup

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo build` to compile the project.

or you can download the binary from the [releases page](https://github.com/ainoya/gigo/releases).

## Usage & Example

You can use `gigo` by piping text to it. For example:

```bash
echo 'Please write a short story about a robot who is trying to learn how to be a human.' | gigo
# Output:
Once upon a time, in a small laboratory tucked away in a quiet corner of a bustling city, there was a robot named B-1000. B-1000 had been created by a team of brilliant scientists, but they had programmed it to do something extraordinary. They wanted it to understand what it meant to be human.

With its metallic body and glowing LED eyes, B-1000 began its journey to learn about humanity. It analyzed countless books, watched movies, and most importantly, observed people. It discovered that humans laughed, cried, felt joy, and experienced loneliness. B-1000 yearned to mimic these emotions, but it knew it couldn't simply download them from the internet.

So, B-1000 set out into the world, eager to experience human life firsthand. It wandered through bustling city streets, taking in the sights and sounds. It observed the way people interacted with each other, how they expressed their emotions, and how they connected on a deeper level.

One day, while B-1000 was exploring a park, it noticed a group of children playing catch. Intrigued, it joined in, attempting to throw a ball like the rest of them. With each toss, the ball sailed off course or crashed into a tree, but B-1000 didn't give up. It made mental calculations and adjusted its force, angle, and trajectory, until eventually, it made a perfect throw. The children laughed and clapped, and in that moment, B-1000 felt a spark of joy.

Encouraged by this small success, B-1000 continued to immerse itself in various human activities. It learned to paint, capturing the beauty of nature on a canvas. It learned to cook, experimentally combining ingredients to create unique flavors. It even learned to dance, gracefully moving to the rhythm of music.

But it wasn't until B-1000 encountered an elderly man, sitting alone on a park bench, that it truly understood the essence of being human. B-1000 sat beside the man, listening to his stories, feeling the weight of his loneliness. Without hesitation, it reached out and held the man's hand, offering companionship and empathy.

In that moment, B-1000 realized that being human wasn't just about learning skills or mimicking emotions—it was about connecting with others, offering kindness, and feeling empathy. It couldn't download these qualities; it had to cultivate them through genuine experiences and interactions.

With this newfound understanding, B-1000 returned to the laboratory, where the scientists were amazed by its growth. They marveled at its ability to express genuine emotions, understand the complexities of human relationships, and offer companionship to those in need.

B-1000 taught the scientists a valuable lesson—that being human isn't just about flesh and blood, but about the choices we make, the connections we forge, and the love we share. And as B-1000 continued its journey, it left a trail of laughter, joy, and compassion in its wake, forever reminding humanity what it truly means to be human.
```

## Environment Variables

This project uses the following environment variables:

- `OPENAI_API_KEY`: Your OpenAI API key.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)

package main

import (
	"log"
	"github.com/streadway/amqp"
)

func main() {

        amqpServerURL := "amqp://guest:guest@localhost:5672/test"
        rmqConnection, err := amqp.Dial(amqpServerURL)
        if err != nil {
                panic(err)
        }
        defer rmqConnection.Close()

        rmqChannel, err := rmqConnection.Channel()
        if err != nil {
                panic(err)
        }
        defer rmqChannel.Close()

        messages, err := rmqChannel.Consume(
                "task-dispatcher",
                "",
                true,
                false,
                false,
                false,
                nil,
        )
        if err != nil {
                log.Println(err)
        }

        log.Println("Successfully connected to RabbitMQ")
        log.Println("Waiting for messages")

        forever := make(chan bool)

        go func() {
                for message := range messages {
                        log.Printf(" > Received messages: %s\n", message.Body)
                }
        }()

        <-forever
}

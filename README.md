# amqpcat
 CLI AMQP producer and consumer

Inpired by https://github.com/edenhill/kafkacat, this is a AMQP producer/consumer used from the CLI. 

The consumer is the initial focus and the first feature I'm working on is to setup a temp queue with generated
name and bind that to `amq.topic` with a given routing-key as this fits very much with how we use AMQP at my job. 

package main

import (
	"fmt"
	"log"
	"os"
	"time"

	"encoding/json"

	"net/http"

	"github.com/joho/godotenv"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

var LoopInterval = time.Second * 5

type ApplicationConfiguration struct {
	MqttHost          string
	MqttPort          string
	LocationLatitude  string
	LocationLongitude string
}

type SunriseSunsetResponse struct {
	Status  string              `json:"status"`
	Results SunriseSunsetResult `json:"results"`
}

type SunriseSunsetResult struct {
	Sunrise                   string `json:"sunrise"`
	Sunset                    string `json:"sunset"`
	SolarNoon                 string `json:"solar_noon"`
	DayLength                 string `json:"day_length"`
	CivilTwilightBegin        string `json:"civil_twilight_begin"`
	CivilTwilightEnd          string `json:"civil_twilight_end"`
	NauticalTwilightBegin     string `json:"nautical_twilight_begin"`
	NauticalTwilightEnd       string `json:"nautical_twilight_end"`
	AstronomicalTwilightBegin string `json:"astronomical_twilight_begin"`
	AstronomicalTwilightEnd   string `json:"astronomical_twilight_end"`
}

func periodicDayphase(conf ApplicationConfiguration, mqtt mqtt.Client) {
	for {
		url := fmt.Sprintf(
			"http://api.sunrise-sunset.org/json?lat=%s&lng=%s&date=%s",
			conf.LocationLatitude,
			conf.LocationLongitude,
			"2021-01-13",
		)

		log.Println("periodicDayPhase from", url)

		res, err := http.Get(url)

		if err != nil {
			log.Println("Failed HTTP Request")
			time.Sleep(LoopInterval)
			continue
		}

		val := SunriseSunsetResponse{}

		dec := json.NewDecoder(res.Body)
		dec.DisallowUnknownFields()

		err = dec.Decode(&val)

		res.Body.Close()

		if err != nil {
			log.Println("Failed JSON Parsing", err)
			time.Sleep(LoopInterval)
			continue
		}

		log.Println(val)

		time.Sleep(LoopInterval)
	}
}

func init() {
	godotenv.Load()
}

func main() {
	conf := ApplicationConfiguration{
		MqttHost:          os.Getenv("MQTT_HOST"),
		MqttPort:          os.Getenv("MQTT_PORT"),
		LocationLatitude:  os.Getenv("LOCATION_LATITUDE"),
		LocationLongitude: os.Getenv("LOCATION_LONGITUDE"),
	}

	opts := mqtt.NewClientOptions()
	opts.AddBroker(fmt.Sprintf("tcp://%s:%s", conf.MqttHost, conf.MqttPort))
	opts.SetClientID("go_test")

	client := mqtt.NewClient(opts)

	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}

	go periodicDayphase(conf, client)

	time.Sleep(time.Second * 60)
}

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button, Flex, Text } from "@chakra-ui/react";
import "./App.css";
import { ask } from "@tauri-apps/api/dialog";
import { sendNotification} from "@tauri-apps/api/notification";

function App() {
  const [time, setTime] = useState(0);
  const [timerStart, setTimerStart] = useState(false);

  const buttons = [
    {
      value : 900,
      display : "15 minutes"
    },
    {
      value: 1800,
      display: "30 minutes"
    },
    {
      value: 3600,
      display: "60 minutes"
    }
  ];

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  }

  const toggleTimer = () => {
    setTimerStart(!timerStart);
  }

  const triggerResetDialog = async () => {
    let shouldReset = await ask("Do you want to reset timer?", {
      title: "pomodoro timer app",
      type: "warning"
    });
    if (shouldReset) {
      setTimerStart(false);
      setTime(900);
    }
  };

  useEffect(() => {
    const interval = setInterval(() => {
      if (timerStart) {
        if (time > 0) {
          setTime(time - 1);
        } else if (time === 0) {
          sendNotification({
            title: `Time's up!`,
            body: `Congrats on completing a session!🎉`,
          });
          clearInterval(interval);
        }
      }
    }, 1000);
    return () => clearInterval(interval);
  }, [timerStart, time]);

  return (
    <div className="App" style={{ height: "100%" }}>
      <Flex
        background="gray.700"
        height="100%"
        alignItems="center"
        flexDirection="column"
      >
        <Text color="white" fontWeight="bold" marginTop="20" fontSize="35">
          my pomodoro timer
        </Text>
        <Text fontWeight="bold" fontSize="7xl" color="white">
        {`${
            Math.floor(time / 60) < 10
              ? `0${Math.floor(time / 60)}`
              : `${Math.floor(time / 60)}`
          }:${time % 60 < 10 ? `0${time % 60}` : time % 60}`}
        </Text>
        <Flex>
          <Button
            background="tomato"
            color="white"
            width="7em"
            onClick={toggleTimer}
          >
            {!timerStart ? "start" : "pause" }
          </Button>
          <Button
            background="blue.300"
            marginX="5"
            onClick={triggerResetDialog}
          >
            reset
          </Button>
        </Flex>
        <Flex marginTop={10}>
          {buttons.map(({value, display}) => (
            <Button
              marginX={4}
              background="green.300"
              color={'white'}
              onClick={() => {
                  setTimerStart(false);
                  setTime(value);
                }
              }
            >
              {display}
            </Button>
          ))}
        </Flex>
      </Flex>
    </div>
  );
}

export default App;

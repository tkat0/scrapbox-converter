import {
  Box,
  Button,
  Divider,
  Flex,
  Heading,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  NumberDecrementStepper,
  NumberIncrementStepper,
  NumberInput,
  NumberInputField,
  NumberInputStepper,
  Spacer,
  Switch,
  Text,
  Tooltip,
  useDisclosure,
} from "@chakra-ui/react";
import React from "react";

export interface Config {
  heading1Mapping: number;
  boldToHeading: boolean;
}

export const defaultConfig: Config = {
  heading1Mapping: 4,
  boldToHeading: false,
};

interface ConfigModalProps {
  config: Config;
  setConfig: (config: Config) => void;
}

export const ConfigModal: React.FC<ConfigModalProps> = (props) => {
  const { isOpen, onOpen, onClose } = useDisclosure();

  const { config, setConfig } = props;

  return (
    <>
      <Button onClick={onOpen} size="sm">
        Configuration
      </Button>

      <Modal onClose={onClose} isOpen={isOpen} isCentered size={"2xl"}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Configuration</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <ConfigHeader
              value={config.heading1Mapping}
              defaultValue={defaultConfig.heading1Mapping}
              setValue={(value) => {
                setConfig({ ...config, heading1Mapping: value });
              }}
            />
            <Divider />
            <ConfigBold
              value={config.boldToHeading}
              defaultValue={defaultConfig.boldToHeading}
              setValue={(value) => {
                setConfig({ ...config, boldToHeading: value });
              }}
            />
          </ModalBody>
          <ModalFooter>
            <Button onClick={onClose}>Close</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
};

interface ConfigProps<T> {
  value: T;
  defaultValue: T;
  setValue: (value: T) => void;
}

function ConfigHeader(props: ConfigProps<number>) {
  const { value, setValue, defaultValue } = props;
  return (
    <ConfigRow
      title={"Heading1 Mapping"}
      descriptions={[`[${"*".repeat(value)} heading] → # heading1`]}
      defaultValue={defaultValue}
      setValue={setValue}
    >
      <NumberInput
        defaultValue={value}
        min={1}
        max={5}
        size="sm"
        onChange={(value) => {
          const v = parseInt(value);
          if (1 <= v && v <= 5) {
            setValue(parseInt(value));
          }
        }}
      >
        <NumberInputField size={1} />
        <NumberInputStepper>
          <NumberIncrementStepper />
          <NumberDecrementStepper />
        </NumberInputStepper>
      </NumberInput>
    </ConfigRow>
  );
}

function ConfigBold(props: ConfigProps<boolean>) {
  const { value, setValue, defaultValue } = props;
  return (
    <ConfigRow
      title={"Convert [* Bold] to Heading"}
      descriptions={[`[* bold] → **bold** or [* header] → ### header`]}
      defaultValue={defaultValue}
      setValue={setValue}
    >
      <Switch
        size="sm"
        isChecked={value}
        onChange={() => {
          setValue(!value);
        }}
      />
    </ConfigRow>
  );
}

interface ConfigRowProps<T> {
  title: string;
  descriptions: string[];
  defaultValue: T;
  setValue: (value: T) => void;
  children: React.ReactNode;
}

function ConfigRow<T>(props: ConfigRowProps<T>) {
  const { title, descriptions, defaultValue, setValue, children } = props;
  return (
    <Flex alignItems={"center"} gap={"2"} m="2">
      <Box>
        <Heading size="sm">{title}</Heading>
        {descriptions.map((d) => {
          return <Text fontSize={"sm"}>{d}</Text>;
        })}
      </Box>
      <Spacer />
      <Box>{children}</Box>
      <Box>
        <Tooltip label={`default: ${defaultValue}`}>
          <Button
            size="sm"
            onClick={() => {
              setValue(defaultValue);
            }}
          >
            Reset
          </Button>
        </Tooltip>
      </Box>
    </Flex>
  );
}

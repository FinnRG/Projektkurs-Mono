import { Group, PasswordInput, Progress } from '@mantine/core';
import { ChangeEventHandler } from 'react';
import zxcvbn from 'zxcvbn';

const getStrength = (password: string) => {
  return zxcvbn(password).score * 25;
};

interface PasswordStrengthProps {
  value: string;
  setValue: ChangeEventHandler;
  mt?: string;
}

const PasswordStrength = ({ value, setValue, mt }: PasswordStrengthProps) => {
  const strength = getStrength(value);

  const bars = Array(4)
    .fill(0)
    .map((_, index) => (
      <Progress
        styles={{ bar: { transitionDuration: '0ms' } }}
        value={
          value.length > 0 && index === 0
            ? 100
            : strength >= ((index + 1) / 4) * 100
            ? 100
            : 0
        }
        color={strength > 80 ? 'teal' : strength > 50 ? 'yellow' : 'red'}
        key={index}
        size={4}
      />
    ));

  return (
    <div>
      <PasswordInput
        value={value}
        onChange={setValue}
        placeholder='Your password'
        label='Password'
        mt={mt}
        required
      />

      <Group spacing={5} grow mt='xs' mb='md'>
        {bars}
      </Group>
    </div>
  );
};

export default PasswordStrength;

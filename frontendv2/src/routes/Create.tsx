import {
  Button,
  Container,
  TextInput,
  Stepper,
  Space,
  Select,
  Group,
} from '@mantine/core';
import { useForm } from '@mantine/hooks';
import { useState } from 'react';
import { createVideo, uploadVideo } from '../client';
import DropzoneWithButton from '../components/Create/Dropzone';
import { Visibility } from '../gen/videos/v1/videos';

const CreateVideo = () => {
  const [id, setId] = useState('');
  const [loaded, setLoaded] = useState(false);

  const [active, setActive] = useState(0);
  const nextStep = () => {
    if (!form.validate()) {
      return;
    }
    if (active == 0) {
      createVideo(form.values)?.then(({ response }) => setId(response.id));
    }
    setActive((current) => (current < 3 ? current + 1 : current));
  };
  const prevStep = () =>
    setActive((current) => (current > 0 ? current - 1 : current));

  const upload = (files: File[]) => {
    uploadVideo(id, files[0]).then((resp) => {
      if (resp.status == 200) {
        setLoaded(true);
      }
    });
  };

  const form = useForm({
    initialValues: {
      title: '',
      description: '',
      visibility: Visibility.UNSPECIFIED,
    },
    validationRules: {
      title: (value) => (value.length > 0 ? true : false),
      visibility: (value) =>
        [Visibility.PRIVATE, Visibility.PUBLIC].includes(value),
    },
  });

  return (
    <>
      <Stepper active={active} breakpoint='sm'>
        <Stepper.Step label='Step 1' description='Set metadata' />
        <Stepper.Step label='Step 2' description='Upload video' />
        <Stepper.Step label='Step 3' description='Share link' />
      </Stepper>

      <Container size={420} my={40}>
        <Space h={'xl'} />
        {active == 0 && (
          <form>
            <TextInput
              required
              label='Title'
              {...form.getInputProps('title')}
            />
            <TextInput
              label='Description'
              mt={'xs'}
              {...form.getInputProps('description')}
            />
            <Select
              label='Visibility'
              placeholder='Click here to select a visibility option'
              data={[
                { value: 'Public', label: 'Public' },
                { value: 'Private', label: 'Private' },
              ]}
              mt={'xs'}
              onChange={(value) =>
                form.setFieldValue(
                  'visibility',
                  value == 'Public' ? Visibility.PUBLIC : Visibility.PRIVATE
                )
              }
              error={form.getInputProps('visibility').error}
            />
          </form>
        )}
        {active == 1 && (
          <DropzoneWithButton onDrop={upload} loaded={loaded} progress={0} />
        )}
        <Space h={'xl'} />
        <Group position='center' mt='xl'>
          <Button variant='default' onClick={prevStep}>
            Back
          </Button>
          <Button onClick={nextStep}>Submit</Button>
        </Group>
      </Container>
    </>
  );
};

export default CreateVideo;

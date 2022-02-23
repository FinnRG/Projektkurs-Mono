import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { Form, Icon } from 'react-bulma-components';
const { Input, Label, Control } = Form;

const FormInputField = (props) => {
  return (
    <>
      <Label>{props.label}</Label>
      <Control>
        <Input
          value={props.value}
          type={props.type}
          onChange={(e) => {
            return props.setValue(e.target.value);
          }}
        />
        <Icon align='left' size='small'>
          <FontAwesomeIcon icon={props.icon} />
        </Icon>
        {props.children}
      </Control>
    </>
  );
};

export default FormInputField;

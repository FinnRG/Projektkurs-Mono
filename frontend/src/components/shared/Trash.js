import { faTrash } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';

const Trash = (props) => {
  const [trashClassName, setTrashClassName] = useState('has-text-grey-light');

  return (
    <FontAwesomeIcon
      onMouseEnter={() => setTrashClassName('has-text-dark')}
      onMouseLeave={() => setTrashClassName('has-text-grey-light')}
      onClick={() => props.onClick()}
      className={trashClassName}
      icon={faTrash}
    />
  );
};
export default Trash;

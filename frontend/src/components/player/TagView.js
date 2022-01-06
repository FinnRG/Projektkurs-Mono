import TagViewElement from '../tag/Tag';

const TagView = (props) => {

    const removeTag = (id) => {
        props.setTags(props.tags.filter((tag) => tag.id !== id));
    }

    return <>
        {props.tags.map((tag, index) => <TagViewElement key={index} name={tag.name} tag_id={tag.id} onSuccess={() => removeTag(tag.id)} />)}
    </>
}

export default TagView;
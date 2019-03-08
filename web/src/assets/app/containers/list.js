import { connect } from 'react-redux';
import FilesList from '../components/List/FilesList.jsx';

const mapStateToProps = (state) => {
	return {
		list_files: state.list_files
	}
}

const FilesList_ = connect(
	mapStateToProps
)(FilesList);


export default FilesList_

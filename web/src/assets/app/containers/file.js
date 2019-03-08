import { connect } from 'react-redux';
import File from '../components/Input/File.jsx';

const mapStateToProps = (state) => {
	return {
		files_inputs: state.files_inputs
	}
}

const File_ = connect(
	mapStateToProps
)(File);


export default File_

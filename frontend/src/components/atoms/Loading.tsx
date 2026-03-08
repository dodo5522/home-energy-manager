import {Box, CircularProgress} from '@mui/material';

const Loading = () => {
  return (
    <Box sx={{display: 'flex', justifyContent: 'center', py: 10}}>
      <CircularProgress size={24}/>
    </Box>
  );
};

export default Loading;

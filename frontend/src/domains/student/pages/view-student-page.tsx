import * as React from 'react';
import { Box, Button, Paper, Tab, Tabs } from '@mui/material';
import { Download } from '@mui/icons-material';
import { useParams } from 'react-router-dom';

import { TabPanel } from '@/components/tab-panel';
import { PageContentHeader } from '@/components/page-content-header';
import { StudentProfile } from '@/components/user-account-profile';
import { getStudentReportUrl } from '@/utils/helpers/student-report';

const tabs = ['Profile'];
export const ViewStudent = () => {
  const { id } = useParams();
  const [tab, setTab] = React.useState(0);

  React.useEffect(() => {
    setTab(0);
  }, []);

  const handleTabChange = (_event: React.SyntheticEvent, index: number) => {
    setTab(index);
  };

  return (
    <>
      <Box sx={{ display: 'flex', mb: 1 }}>
        <Box sx={{ ml: 'auto' }}>
          <Button
            size='small'
            color='primary'
            variant='contained'
            startIcon={<Download />}
            component='a'
            href={id ? getStudentReportUrl(id) : undefined}
            target='_blank'
            rel='noopener'
            disabled={!id}
          >
            Download PDF Report
          </Button>
        </Box>
      </Box>
      <PageContentHeader heading='Account Details' />
      <Box component={Paper} sx={{ p: 1 }}>
        <Tabs
          variant='scrollable'
          value={tab}
          onChange={handleTabChange}
          sx={{ borderRight: 1, borderColor: 'divider' }}
        >
          {tabs.map((tab) => (
            <Tab key={tab} label={tab} />
          ))}
        </Tabs>
        <Box sx={{ display: 'flex', flexGrow: 1 }}>
          <TabPanel value={tab} index={0}>
            <StudentProfile id={id} />
          </TabPanel>
        </Box>
      </Box>
    </>
  );
};

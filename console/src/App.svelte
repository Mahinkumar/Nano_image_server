<script lang="ts">
  import { Button, Pane, TabGroup, TabPage } from 'svelte-tweakpane-ui';
  import { Monitor } from 'svelte-tweakpane-ui';

  let booleanToMonitor = false;
  let stringToMonitor = 'Reticulating';
  let numberToMonitor = 85;

  setInterval(() => {
    numberToMonitor = Math.random() * 100;
  }, 100);

  setInterval(() => {
    booleanToMonitor = !booleanToMonitor;
    stringToMonitor = [...stringToMonitor].reverse().join('');
}, 1000);
</script>

<Pane width={window.innerWidth} x={0} y={0} position={"fixed"} userExpandable={false}>
  <TabGroup>
    <TabPage title="Console">
    </TabPage>
    <TabPage title="Configuration">
    </TabPage>
    <TabPage title="Image Browser">
    </TabPage>
    <TabPage title="Editor">
    </TabPage>
  </TabGroup>
</Pane>


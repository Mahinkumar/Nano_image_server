<script lang="ts">
  import {Pane} from 'tweakpane';
  const pane = new Pane();

  const PARAMS = {
  percentage: 50,
  theme: 'dark',
  Parameter: 5,
  Filter: 'blur',
  color: '#ff0055',
  };


  
  pane.addBinding(PARAMS, 'Filter');
  pane.addBinding(PARAMS, 'color');
  pane.addBinding(PARAMS, 'Parameter');


  // `min` and `max`: slider
  pane.addBinding(
    PARAMS, 'percentage',
    {min: 0, max: 100, step: 10}
  );

  // `options`: list
  pane.addBinding(
    PARAMS, 'theme',
    {options: {Dark: 'dark', Light: 'light'}}
  );
</script>


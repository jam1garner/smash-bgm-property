# bgm_property_lib

A Rust library for reading and writing `bgm_property.bin` files from Super Smash Bros. Ultimate.

## bgm_property_yaml

A command-line program for creating and editing `bgm_property.bin` files using YAML. Drag and drop a `bgm_property.bin` file onto the executable to create a YAML file. Drag and drop a properly structured YAML file onto the executable to create a `bgm_property.bin` file. YAML files are text files, so they can be viewed and edited in any text editor.

Sample output from a `bgm_property.bin` file:

```yaml
entries:
- name_id: a01_smb_chijyou
  loop_start_ms: 31307
  loop_start_sample: 1502745
  loop_end_ms: 109769
  loop_end_sample: 5268902
  total_time_ms: 109774
  total_samples: 5269164
- name_id: a02_smb_chika
  loop_start_ms: 10189
  loop_start_sample: 489058
  loop_end_ms: 85291
  loop_end_sample: 4093989
  total_time_ms: 85292
  total_samples: 4094033
```

### Usage

The latest prebuilt binary for Windows is available in [Releases](https://github.com/jam1garner/smash-bgm-property/releases/latest).

Download the latest set of [labels](https://github.com/ultimate-research/param-labels/blob/master/bgm_property/Labels.txt) and have them placed beside the executable when dragging and dropping or include them in the command when converting to YAML. Missing labels will result in all `name_id` values appearing as hashes.

`bgm_property_yaml <input> [output]`<br>
`bgm_property_yaml <input> [output] [label]`<br>
`bgm_property_yaml bgm_property.bin bgm_property.yaml`<br>
`bgm_property_yaml bgm_property.bin bgm_property.yaml Labels.txt`<br>
`bgm_property_yaml bgm_property.yaml bgm_property.bin`<br>

<table>
<tr>
<td><img src="./docs/logo_partial_white_bg.png" width="256px"> 
</td>
<td>
<h1>VTuber Fun ToolKit (VTFTK)</h1>
<p><b>WIP</b> App for VTubers to create interactive experiences with their viewers
</p>
</td>
</tr>
</table>

## 

Create items that can be thrown at you, sounds that can be played, and trigger VTube studio. All using events from twitch such as redeems, commands, subscriptions, follows, raids, bits

Still early stages and work in progress, but it does work and throwables / sounds can be created then triggered by events.

Aiming to be a tailored and improved version of [KBonk](https://github.com/typeou/karasubonk) more suited to my needs and wants.

![Throwables](./docs/throwables.png)

## Scripting 

Custom scripting for handling events and creating custom interactive commands

> **NOTE**
> Currently only the "chat" event is exposed along with some APIs for working with persisted key value pairs and some Twitch and HTTP APIs


Built on `deno_core` providing scripting through JavaScript.


![Scripting](./docs/scripts.png)

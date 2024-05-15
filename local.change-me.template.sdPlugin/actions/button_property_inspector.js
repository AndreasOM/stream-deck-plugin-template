console.log('Property Inspector loaded', $PI);


$PI.onConnected(jsn => {
    console.log('Property Inspector connected', jsn);
    console.log(jsn.actionInfo.payload.settings);
    Object.entries(jsn.actionInfo.payload.settings).forEach(([key, value]) => {
        console.log('setting', key, value);
        const el = document.getElementById(key);
        if(el) {
            el.value = value;
        }
    });

    let actionUUID = $PI.actionInfo.action;
    $PI.onSendToPropertyInspector(actionUUID, jsn => {
        console.log('onSendToPropertyInspector', jsn);
    });
});

const changed = () => {
    console.log('changed', event, event.target, event.target.value);
    $PI.setSettings({[event.target.id]: event.target.value});
};

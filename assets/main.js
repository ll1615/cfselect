document.addEventListener("DOMContentLoaded", () => {
    const $ip_ranges = document.querySelector("#ip_ranges");
    $ip_ranges.value = `173.245.48.0/20
103.21.244.0/22
103.22.200.0/22
103.31.4.0/22
141.101.64.0/18
108.162.192.0/18
190.93.240.0/20
188.114.96.0/20
197.234.240.0/22
198.41.128.0/17
162.158.0.0/15
104.16.0.0/13
104.24.0.0/14
172.64.0.0/13
131.0.72.0/22`;

    // 填充已优选IP记录
    const radio = document.createElement("input");
    radio.type = "radio";
    radio.name = "selected_ip";
    radio.classList.add("radio", "border", "border-black");
    const cell_cbx = document.createElement("th");
    cell_cbx.appendChild(document.createElement("label"))
            .appendChild(radio);
    const $selected_ips = document.querySelector("#selected_ips");
    const $selected_ips_body = $selected_ips.querySelector("tbody");
    const fill_selected_ips = async function() {
        let response = await fetch("/api/ip/select");
        let obj = await response.json();
        display_message(obj);

        if (obj?.code != 0) {
            return;
        }

        // 清空表格
        $selected_ips_body.innerHTML = "";

        const fragment = document.createDocumentFragment();
        for (let record of obj?.data) {
            let row = document.createElement("tr");
            row.appendChild(cell_cbx.cloneNode(true));
            for (let field of record) {
                let cell = document.createElement("td");
                cell.innerText = field;
                row.appendChild(cell);
            }
            fragment.appendChild(row);
        }
        $selected_ips_body.appendChild(fragment);

    }

    $alert_msg = document.querySelector("#alert_msg");
    $alert_msg_span = $alert_msg.querySelector("span");
    const display_message = async function(obj) {
        if (obj?.code == 0) {
            $alert_msg.classList.remove("invisible", "alert-error");
            $alert_msg.classList.add("visible", "alert-success");
            $alert_msg_span.innerText = "请求成功！！";

            setTimeout(() => {
                $alert_msg.classList.remove("visible", "alert-success");
                $alert_msg.classList.add("invisible");
            }, 3000);

            return;
        }

        $alert_msg.classList.remove("invisible", "alert-success");
        $alert_msg.classList.add("visible", "alert-error");
        $alert_msg_span.innerText = obj?.message || "Unknown error";
    }

    // 加载状态
    $loading_status = document.querySelector("#loading_status");

    document.querySelector("#btn_select").addEventListener("click", async (evt) => {
        if (checking_status) {
            return;
        }

        const req = $ip_ranges.value.trim().split("\n");
        console.log(JSON.stringify(req));
        let response = await fetch("/api/ip/select", {
            method: "POST",
            body: JSON.stringify(req),
            headers: {
                "Content-Type": "application/json"
            }
        });
        let obj = await response.json();
        display_message(obj);

        if (obj?.code == 0) {
            $loading_status.classList.replace("invisible", "visible");
            check_status();
        }
    });

    let checking_status = false;
    const check_status = async function() {
        if (checking_status) {
            return;
        }

        checking_status = true;
        for (let i = 0; i < 1000; i++) {
            let response = await fetch("/api/ip/select/status");
            let obj = await response.json();
            console.log(`checking status: ${obj?.data}`);
            if (obj?.code == 0 && (obj?.data == "Success" || obj?.data == "Pending")) {
                fill_selected_ips();

                $loading_status.classList.replace("visible", "invisible");
                checking_status = false;
                return;
            }

            await sleep(2000);
        }
    }

    const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

    // 检查状态并调用填充已优选IP表
    $loading_status.classList.replace("invisible", "visible");
    check_status();

    $sync_dns = document.querySelector("#sync_dns");
    $sync_dns.addEventListener("click", async (evt) => {
        $alert_msg.classList.replace("visible", "invisible");

        const $checked = $selected_ips.querySelector("input:checked");
        if (!$checked) {
            display_message({ message: "请至少选择一个IP" });
            return;
        }

        const ip = $checked.parentNode.parentNode.nextSibling.textContent;
        if (!is_ipv4(ip)) {
            display_message({ message: "IP格式错误" });
            return;
        }

        $loading_status.classList.replace("invisible", "visible"); // 显示加载状态
        let response = await fetch("/api/dns/sync", {
            method: "POST",
            body: JSON.stringify({ ip }),
            headers: {
                "Content-Type": "application/json"
            }
        });

        let obj = await response.json();
        display_message(obj);

        obj?.code == 0 && $loading_status.classList.replace("visible", "invisible"); // 隐藏加载状态
    });

    const is_ipv4 = (ip) => {
        const secs = ip.split(".");
        if (secs.length != 4) {
            return false;
        }
        return secs.every(e => !(isNaN(e) || e < 0 || e > 255));
    }
});
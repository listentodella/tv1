<template>
    <div class="home">
        <h1>Welcome to the Home Page</h1>
        <p>This is a simple Vue.js application.</p>
    </div>
</template>



<script lang="ts" setup name="Home">

</script>


<style scoped>
/* 导航项基本样式 */
.nav-item {
    position: relative;
    display: block;
    height: 48px;
    border-radius: 0.75rem;
    color: inherit;
    text-decoration: none;
    overflow: hidden;
    width: 100%;
    transition: background-color 0.3s ease, box-shadow 0.3s ease, color 0.3s ease;
    cursor: pointer;
    background: none;
    border: none;
    text-align: left;
    padding: 0;
    user-select: none;
    /* 防止导航项内容被选中 */
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
}

/* 切换按钮特殊样式 */
.toggle-btn {
    font-family: inherit;
    outline: none;
}

/* 图标盒子样式 - 使用绝对定位固定在左侧 */
.icon-box {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
}

/* 图标样式 */
.material-icons {
    font-size: 24px;
    transition: transform 0.2s, color 0.3s;
    user-select: none;
    /* 防止文本被选中 */
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    pointer-events: none;
    /* 防止图标响应鼠标事件 */
    -webkit-user-drag: none;
    /* 防止拖拽 */
}

/* 标签文本样式 */
.label {
    display: block;
    position: absolute;
    left: 48px;
    top: 50%;
    transform: translateY(-50%);
    white-space: nowrap;
    transition: opacity 0.3s;
    opacity: 1;
    user-select: none;
    /* 防止文本被选中 */
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
}

.hidden-label {
    opacity: 0;
}

/* 弱化的强调色，用于鼠标悬停状态 */
.nav-item:hover,
.toggle-btn:hover {
    background-color: rgba(74, 108, 247, 0.2) !important;
    color: #4a6cf7 !important;
    box-shadow: 0 0 0 1px rgba(74, 108, 247, 0.3) !important;
}

/* 强调色，用于当前选中的页面 */
.nav-item.router-link-active {
    background-color: rgba(74, 108, 247, 0.3) !important;
    color: #4a6cf7 !important;
    font-weight: 500 !important;
    box-shadow: 0 0 0 2px rgba(74, 108, 247, 0.5) !important,
        inset 4px 0 0 #4a6cf7 !important,
        0 4px 8px rgba(0, 0, 0, 0.1) !important;
}

/* 悬停和激活状态下的图标效果 */
.nav-item:hover .material-icons,
.toggle-btn:hover .material-icons {
    color: #4a6cf7 !important;
}

.nav-item.router-link-active .material-icons {
    color: #4a6cf7 !important;
    transform: scale(1.15);
    text-shadow: 0 0 8px rgba(74, 108, 247, 0.5);
}

/* 提示框样式 */
.tooltip {
    position: absolute;
    left: 48px;
    top: 50%;
    transform: translateY(-50%);
    background-color: #4a6cf7 !important;
    color: white !important;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 14px;
    white-space: nowrap;
    z-index: 1000;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s;
    box-shadow: 0 3px 10px rgba(74, 108, 247, 0.4);
    font-weight: normal;
    visibility: hidden;
}

.icon-box:hover .tooltip {
    opacity: 1;
    visibility: visible;
}

/* 确保在暗黑模式下颜色仍然可见 */
@media (prefers-color-scheme: dark) {

    .nav-item:hover,
    .toggle-btn:hover,
    .nav-item.router-link-active {
        background-color: rgba(100, 130, 255, 0.25) !important;
        color: #90a8ff !important;
    }

    .nav-item:hover .material-icons,
    .toggle-btn:hover .material-icons,
    .nav-item.router-link-active .material-icons {
        color: #90a8ff !important;
    }

    .nav-item.router-link-active {
        box-shadow: 0 0 0 2px rgba(100, 130, 255, 0.5) !important,
            inset 4px 0 0 #90a8ff !important,
            0 4px 8px rgba(0, 0, 0, 0.2) !important;
    }

    .tooltip {
        background-color: #6482ff !important;
    }
}

/* 添加禁用导航项样式 */
.disabled-nav-item {
    opacity: 0.6;
    cursor: not-allowed;
    pointer-events: none;
}

.disabled-nav-item:hover {
    background-color: transparent !important;
    color: inherit !important;
    box-shadow: none !important;
}

.disabled-nav-item .material-icons {
    color: inherit !important;
    transform: none !important;
}
</style>
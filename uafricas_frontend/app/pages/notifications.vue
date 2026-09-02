<script setup lang="ts">
import { iconeNotification } from '~/mocks/notifications'

/**
 * Notifications : la page complète derrière la cloche.
 *
 * Le lien « Voir toutes les notifications » du menu déroulant pointait sur
 * `/notifications`, qui n'existait pas : 404 à chaque clic. La cloche, elle,
 * n'affiche que les premières.
 *
 * Aucun endpoint neuf : la même route paginée que la cloche, avec en plus le
 * filtre par type que l'API accepte déjà (`?type=`) et qui n'était exploité
 * nulle part.
 */
definePageMeta({ middleware: 'auth', layout: false })

useHead({ title: 'Notifications | AfricanS' })

interface Notification {
  id: string
  type: string
  message: string
  lien_action?: string | null
  lu: boolean
  created_at: string
}

const { nbNonLues, compteurNonLues, listerNotifications, marquerLue, toutMarquerLu } = useNotifications()

const notifications = ref<Notification[]>([])
const chargement = ref(false)
const page = ref(1)
const typeFiltre = ref('')
const finDeListe = ref(false)

/** Les types que l'API sait produire, avec leur libellé. Un type inconnu
 *  reste affichable : il tombe sur la cloche générique, jamais dans le vide. */
const TYPES: Record<string, string> = {
  matching: 'Correspondances',
  collaboration: 'Collaboration',
  invitation: 'Invitations',
  contact: 'Contacts',
  systeme: 'Système',
  evenement_direct_demarre: 'Événements en direct',
  'engagement.niveau_atteint': 'Niveaux',
  'engagement.badge_debloque': 'Badges',
}

const charger = async (reinitialiser = false) => {
  if (reinitialiser) {
    page.value = 1
    finDeListe.value = false
    notifications.value = []
  }
  chargement.value = true
  try {
    const r = await listerNotifications(page.value, typeFiltre.value || undefined)
    if (r.success && r.data) {
      notifications.value = reinitialiser ? r.data : [...notifications.value, ...r.data]
      // Le backend ne renvoie pas de total : une page vide signale la fin.
      finDeListe.value = r.data.length === 0
    }
  }
  finally {
    chargement.value = false
  }
}

const chargerPlus = () => {
  page.value += 1
  charger()
}

const ouvrir = async (n: Notification) => {
  if (!n.lu) {
    await marquerLue(n.id, (n as { source?: 'arbre' | 'africonnect' }).source)
    n.lu = true
  }
  if (n.lien_action) navigateTo(n.lien_action)
}

const toutLire = async () => {
  await toutMarquerLu()
  notifications.value = notifications.value.map(n => ({ ...n, lu: true }))
}

watch(typeFiltre, () => charger(true))

onMounted(() => {
  compteurNonLues()
  charger(true)
})

const dateRelative = (iso: string): string => {
  const minutes = Math.floor((Date.now() - new Date(iso).getTime()) / 60000)
  if (minutes < 1) return "à l'instant"
  if (minutes < 60) return `il y a ${minutes} min`
  const heures = Math.floor(minutes / 60)
  if (heures < 24) return `il y a ${heures} h`
  const jours = Math.floor(heures / 24)
  if (jours < 7) return `il y a ${jours} j`
  return new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
}
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Notifications"
        sous-titre="Tout ce qui vous concerne sur la plateforme"
        image="/images/africans/heros/hero-accueil.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Notifications' }]">
        <template #action>
          <AfricansBouton
            v-if="nbNonLues > 0"
            variante="secondaire"
            icone="fa-solid fa-check"
            @click="toutLire"
          >
            Tout marquer comme lu
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-5">
      <div v-if="chargement && !notifications.length" class="flex flex-col gap-3">
        <div v-for="n in 5" :key="n" class="h-20 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="!notifications.length" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-bell" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ typeFiltre ? 'Aucune notification de ce type' : 'Aucune notification' }}
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          Vous serez prévenu ici des correspondances, invitations et événements qui vous concernent.
        </p>
      </div>

      <template v-else>
        <button
          v-for="n in notifications"
          :key="n.id"
          type="button"
          class="flex w-full items-start gap-4 rounded-[10px] border bg-white p-4 text-left transition hover:border-af-chocolat"
          :class="n.lu ? 'border-af-bordure' : 'border-af-chocolat/30 bg-af-chocolat/5'"
          @click="ouvrir(n)"
        >
          <span class="grid size-10 shrink-0 place-items-center rounded-full bg-af-fond text-af-chocolat">
            <font-awesome-icon :icon="['fas', iconeNotification(n.type_ as any)]" />
          </span>

          <span class="flex min-w-0 flex-1 flex-col gap-1">
            <span class="text-[14px]/[1.4] text-af-encre" :class="!n.lu && 'font-bold'">
              {{ n.message }}
            </span>
            <span class="flex flex-wrap items-center gap-x-3 text-[12px]/[1.4] text-af-atone">
              <span>{{ TYPES[n.type_] ?? n.type_ }}</span>
              <span>·</span>
              <span>{{ dateRelative(n.created_at) }}</span>
            </span>
          </span>

          <!-- Pastille de non-lu : le fond teinté suffit à l'œil, pas au
               daltonien ni au lecteur d'écran. -->
          <span v-if="!n.lu" class="mt-1 size-2.5 shrink-0 rounded-full bg-af-chocolat">
            <span class="sr-only">Non lue</span>
          </span>
        </button>

        <div v-if="!finDeListe" class="flex justify-center pt-2">
          <AfricansBouton
            variante="secondaire"
            :desactive="chargement"
            :tourne="chargement"
            :icone="chargement ? 'fa-solid fa-spinner' : 'fa-solid fa-arrow-down'"
            @click="chargerPlus"
          >
            {{ chargement ? 'Chargement…' : 'Charger plus' }}
          </AfricansBouton>
        </div>
      </template>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtrer" icone="fa-solid fa-sliders" action-libelle="Tout" @action="typeFiltre = ''">
        <div class="flex flex-col">
          <button
            v-for="(libelle, cle) in TYPES"
            :key="cle"
            type="button"
            class="flex items-center gap-3 rounded-[10px] px-3 py-2 text-left text-[14px]/[1.4] transition hover:bg-af-fond"
            :class="typeFiltre === cle ? 'font-bold text-af-chocolat' : 'text-af-corps'"
            @click="typeFiltre = cle"
          >
            <font-awesome-icon :icon="['fas', iconeNotification(cle as any)]" class="w-4 shrink-0" />
            {{ libelle }}
          </button>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Non lues" icone="fa-solid fa-bell">
        <p class="text-[14px]/[1.4] text-af-corps">
          <span class="text-[20px]/[1.4] font-bold text-af-chocolat">{{ nbNonLues }}</span>
          notification{{ nbNonLues > 1 ? 's' : '' }} en attente de lecture.
        </p>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>
